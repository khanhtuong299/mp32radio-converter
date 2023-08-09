use std::f32::consts::PI;
use hound::WavWriter;
use minimp3::{Decoder, Frame, Error};
use std::fs::File;

const TARGET_SAMPLE_RATE:u32 = 8096;
const TREBLE_GAIN: f32 = 26.0;
const BASS_GAIN:f32 = 1.0;
struct BassTrebleFilter {
    sample_rate:f32,
    gain_bass:f32, gain_treble:f32,
    slope:f32, hz_bass:f32, hz_treble:f32,
    a0_bass:f32, a1_bass:f32, a2_bass:f32, b0_bass:f32, b1_bass:f32, b2_bass:f32,
    a0_treble:f32, a1_treble:f32, a2_treble:f32, b0_treble:f32, b1_treble:f32, b2_treble:f32,

    xn1_bass:f32, xn2_bass:f32, yn1_bass:f32, yn2_bass:f32,
    xn1_treble:f32, xn2_treble:f32, yn1_treble:f32, yn2_treble:f32,
}

impl BassTrebleFilter {
    fn new(gain_bass:f32, gain_treble: f32, sample_rate: f32) -> Self {
        BassTrebleFilter{
            sample_rate,
            gain_bass:10.0_f32.powf(gain_bass/20.0),
            gain_treble:10.0_f32.powf(gain_treble/20.0),
            slope:0.4, hz_bass:250.0, hz_treble:4000.0,
            
            a0_bass:1.0, a1_bass:0.0, a2_bass:0.0, b0_bass:0.0, b1_bass:0.0, b2_bass:0.0,
            xn1_bass:0.0, xn2_bass:0.0, yn1_bass:0.0, yn2_bass:0.0,

            a0_treble:1.0, a1_treble:0.0, a2_treble:0.0, b0_treble:0.0, b1_treble:0.0, b2_treble:0.0,
            xn1_treble:0.0, xn2_treble:0.0, yn1_treble:0.0, yn2_treble:0.0,
        }
    }

    fn coefficients(&mut self){

        let w_bass:f32 = 2.0 *  PI * self.hz_bass / self.sample_rate;
        let a_bass:f32 = (10.0_f32.ln() * self.gain_bass / 40.0).exp(); 
        let b_bass:f32 = ((a_bass * a_bass + 1.0) / self.slope - (a_bass - 1.0)*(a_bass - 1.0)).sqrt();

        self.b0_bass = a_bass * ((a_bass + 1.0) - (a_bass - 1.0) * w_bass.cos() + b_bass * w_bass.sin());
        self.b1_bass = 2.0 * a_bass * ((a_bass - 1.0) - (a_bass + 1.0) * w_bass.cos());
        self.b2_bass = a_bass * ((a_bass + 1.0) - (a_bass - 1.0) * w_bass.cos() - b_bass * w_bass.sin());
        self.a0_bass = (a_bass + 1.0) + (a_bass - 1.0) * w_bass.cos() + b_bass * w_bass.sin();
        self.a1_bass = -2.0 * ((a_bass - 1.0) + (a_bass + 1.0) * w_bass.cos());
        self.a2_bass = (a_bass + 1.0) + (a_bass - 1.0) * w_bass.cos() - b_bass * w_bass.sin();


        let w_treble:f32 = 2.0 *  PI * self.hz_treble / self.sample_rate;
        let a_treble:f32 = (10.0_f32.ln() * self.gain_treble / 40.0).exp(); 
        let b_treble:f32 = ((a_treble * a_treble + 1.0) / self.slope - (a_treble - 1.0)*(a_treble - 1.0)).sqrt();

        self.b0_treble = a_treble * ((a_treble + 1.0) + (a_treble - 1.0) * w_treble.cos() + b_treble * w_treble.sin());
        self.b1_treble = -2.0 * a_treble * ((a_treble - 1.0) + (a_treble + 1.0) * w_treble.cos());
        self.b2_treble = a_treble * ((a_treble + 1.0) + (a_treble - 1.0) * w_treble.cos() - b_treble * w_treble.sin());
        self.a0_treble = (a_treble + 1.0) - (a_treble - 1.0) * w_treble.cos() + b_treble * w_treble.sin();
        self.a1_treble = 2.0 * ((a_treble - 1.0) - (a_treble + 1.0) * w_treble.cos());
        self.a2_treble = (a_treble + 1.0) - (a_treble - 1.0) * w_treble.cos() - b_treble * w_treble.sin();

    }

    fn amplify_treble(&mut self, input: f32) -> f32 {

        let output:f32 = (
            self.b0_treble*input + self.b1_treble*self.xn1_treble + self.b2_treble*self.xn2_treble -
            self.a1_treble*self.yn1_treble - self.a2_treble*self.yn2_treble
        ) / self.a0_treble;

        self.xn2_treble = self.xn1_treble;
        self.xn1_treble = input;
        self.yn2_treble = self.yn1_treble;
        self.yn1_treble = output;

        if output > (i16::MAX-1) as f32 {
            return (i16::MAX-2) as f32
        } else if output < (i16::MIN+1) as f32 {
            return (i16::MIN+2) as f32
        }

        output
    }

    fn amplify_bass(&mut self, input: f32) -> f32{

        let output:f32 = (
            self.b0_bass*input + self.b1_bass*self.xn1_bass + self.b2_bass*self.xn2_bass -
            self.a1_bass*self.yn1_bass - self.a2_bass*self.yn2_bass
        )/self.a0_bass;

        self.xn2_bass = self.xn1_bass;
        self.xn1_bass = input;
        self.yn2_bass = self.yn1_bass;
        self.yn1_bass = output;

        if output > (i16::MAX-1) as f32 {
            return (i16::MAX-2) as f32
        } else if output < (i16::MIN+1) as f32 {
            return (i16::MIN+2) as f32
        }

        output
    }
}

pub fn to_radio(file_path: &str, song_name: String) {

    let mut decoder: Decoder<File> = Decoder::new(File::open(file_path).unwrap());

    let mut input_channels = 2;
    let mut input_sample_rate = 0;

    let mut samples:Vec<i16> = Vec::new();
    loop {
        match decoder.next_frame() {
            Ok(Frame { data, sample_rate, channels, .. }) => {
                samples.extend(data);
                input_channels = channels;
                input_sample_rate = sample_rate;
            },
            Err(Error::Eof) => break,
            Err(e) => panic!("{:?}", e),
        }
    }

    let sample_rate = input_sample_rate as f32;

    let mut hp_filter = BassTrebleFilter::new(BASS_GAIN, TREBLE_GAIN, sample_rate);
    hp_filter.coefficients();
    
    let data:Vec<_> = samples.iter().map(|&x| x as f32).collect();

    let mut left :Vec<f32> = Vec::new();
    let mut right:Vec<f32> = Vec::new();

    for (i, item ) in data.iter().enumerate() {
        if i%2 == 0 {
            left.push(*item);
        } else {
            right.push(*item);
        }
    }

    let hleft:Vec<f32> = left.iter().map(|&x| {
        let out = hp_filter.amplify_treble(x);
        hp_filter.amplify_bass(out)

    }).collect();
    let hright:Vec<f32> = right.iter().map(|&x| {
        let out = hp_filter.amplify_treble(x);
        hp_filter.amplify_bass(out)
    }).collect();

    let lefts = wav_io::resample::linear(hleft, 1, input_sample_rate as u32, TARGET_SAMPLE_RATE);
    let rights = wav_io::resample::linear(hright, 1, input_sample_rate as u32, TARGET_SAMPLE_RATE);


    let mut write_radio = WavWriter::create(
        format!("../radio_out/radio_{}.wav", song_name),
        hound::WavSpec { channels: input_channels as u16, sample_rate: TARGET_SAMPLE_RATE, bits_per_sample: 16, sample_format: hound::SampleFormat::Int },
    ).unwrap();

    for i in 0..lefts.len() {
        write_radio.write_sample((lefts[i]) as i32).unwrap();
        write_radio.write_sample((rights[i]) as i32).unwrap();
    }
}