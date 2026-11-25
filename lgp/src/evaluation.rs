use crate::execution::execute_programme_64b;
use crate::interpreteur::decode_instruction_64b;

use env::{Environment, cartpole::CartPole, pcgrng::PCGRng};
///episode = simulation jusqua la fin
///

/// eval d'un episode
///
pub fn evaluate_episode(program: u64, env: &mut CartPole, max_steps: usize) -> f32 {
    let instructions = decode_instruction_64b(program);
    let mut score = 0.0;
    let mut registre = [0.0; 4];

    let mut rng = PCGRng::seed(0);
    let mut state = env.reset(&mut rng); //retourn l'etat

    for _ in 0..max_steps {
        //les registe a zero de base ?
        let registers = execute_programme_64b(&instructions, &mut registre, &state); // tab de 5 pour chaq prog

        let output = registers[0];

        let action = if output >= 0.0 { 1.0 } else { -1.0 };

        let (next_state, reward, done) = env.step(action, &mut rng);

        score += reward;
        state = next_state;

        if done {
            break;
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use env::cartpole::CartPole;

    #[test]
    fn test_evaluate_episode() {
        let mut cartpole = CartPole::default(); // env cartpole

        let program: u64 = 0;
        let score = evaluate_episode(program, &mut cartpole, 500);
        println!("Score obtenu : {}", score);

        let program: u64 = 0b1100110010000110001110011101110101100010100100000011001101001111;
        let score = evaluate_episode(program, &mut cartpole, 500);
        println!("Score obtenu : {}", score);
    }
}
