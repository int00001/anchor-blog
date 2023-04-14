import * as anchor from '@coral-xyz/anchor';
import { IDL } from '../../../target/types/anchor_blog';

export const getProgram = (provider: anchor.Provider, programId: string) =>
  new anchor.Program(IDL, programId, provider);
