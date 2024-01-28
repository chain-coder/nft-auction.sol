use borsh:: {BorshDeserialize};
use solana_program:: {program_error::ProgramError};

pub enum MovieInstruction {
  AddMovieReview {
    title : String,
    rating: u8,
    description: String
  }
}

#[derive(BorshDeserialize)]
struct MovieReviewPayload {
  title: String,
  rating: u8,
  description: String
}

impl MovieInstruction {
  pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
    
  }
}