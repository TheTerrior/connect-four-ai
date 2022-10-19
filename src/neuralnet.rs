pub struct NeuralNet {
    shape: Vec<i32>,
    weights: Vec<f32>,    // a list of weight matrices
}

impl NeuralNet {
    pub fn from(shape: Vec<i32>, weights: Vec<f32>) -> NeuralNet {
        return NeuralNet { shape, weights }
    }

    // may change this
    pub fn genome_to_weights(shape: &Vec<u32>, genome: Vec<u8>) -> Option<Vec<f32>> {
        let size: u32 = shape.iter().product();
        if usize::try_from(size).unwrap() == genome.len() {     //if the sizes match up
            let mut ret: Vec<f32> = Vec::with_capacity(genome.len());
            for i in (0..genome.len()).step_by(4) {
                ret[i*4] = (genome[i]<<24 + genome[i+1]<<16 + genome[i+2]<<16 + genome[i+3]) as f32;
            }
            return Some(ret);
        } else {
            return None;
        }
    }

    pub fn weights_to_genome(weights: Vec<f32>) {
        let ret: Vec<u8> = Vec::with_capacity(weights.len() * 4);
    }
}