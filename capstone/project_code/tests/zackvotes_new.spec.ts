const anchor = require('@coral-xyz/anchor');
const { SystemProgram, PublicKey } = anchor.web3;

describe('zackvotes_new', () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);
  const program = anchor.workspace.Zackvotes_new;

  let PID;

  it('Initializes and creates a poll', async () => {
    const user = provider.wallet;

    // Derive the PDA for the counter account
    const [counterPda] = await PublicKey.findProgramAddress(
      [Buffer.from('counter')],
      program.programId
    );

    // Attempt to fetch the counter account; if it doesn't exist, initialize it
    let counter;
    try {
      counter = await program.account.counter.fetch(counterPda);
      console.log('Counter account already exists with count:', counter.count.toString());
    } catch (err) {
      console.log('Counter account does not exist. Initializing...');
      await program.rpc.initialize({
        accounts: {
          user: user.publicKey,
          counter: counterPda,
          systemProgram: SystemProgram.programId,
        },
      });
      // Fetch the counter after initialization
      counter = await program.account.counter.fetch(counterPda);
      console.log('Counter initialized with count:', counter.count.toString());
    }

    // Predict the next poll ID (PID) for PDA derivation
    PID = counter.count.add(new anchor.BN(1)); // Counter value after increment
    const [pollPda] = await PublicKey.findProgramAddress(
      [PID.toArrayLike(Buffer, 'le', 8)], // Seed based on the new poll id
      program.programId
    );

    const description = `Test Poll #${PID.toString()}`;
    // Set the start time 10 seconds in the past to ensure the poll is active
    const start = new anchor.BN(Math.floor(Date.now() / 1000) - 10);
    const end = new anchor.BN(Math.floor(Date.now() / 1000) + 86400);

    // Define poll options to be used in the poll's options vector.
    const optionNames = ["Option 1", "Option 2", "Option 3"];

    // Call createPoll with the extra optionNames parameter
    await program.rpc.createPoll(description, start, end, optionNames, {
      accounts: {
        user: user.publicKey,
        poll: pollPda,
        counter: counterPda,
        systemProgram: SystemProgram.programId,
      },
    });

    // Verify that the poll was created with the correct data
    const poll = await program.account.poll.fetch(pollPda);
    console.log('Poll:', poll);
  });

  it('Votes for a candidate', async () => {
    const user = provider.wallet;

    // Recompute PID to match the previous poll ID
    const [counterPda] = await PublicKey.findProgramAddress(
      [Buffer.from('counter')],
      program.programId
    );

    const counter = await program.account.counter.fetch(counterPda);
    PID = counter.count; // Use current counter value as PID

    const [pollPda] = await PublicKey.findProgramAddress(
      [PID.toArrayLike(Buffer, 'le', 8)], // Same derivation logic
      program.programId
    );

    console.log('Voting on Poll ID:', PID.toString());
    console.log('Poll PDA:', pollPda.toBase58());

    const [voterPda] = await PublicKey.findProgramAddress(
      [
        Buffer.from('voter'),
        PID.toArrayLike(Buffer, 'le', 8),
        user.publicKey.toBuffer(),
      ],
      program.programId
    );

    // Perform the vote; here we vote for option 0 (i.e. "Option 1")
    await program.rpc.vote(PID, new anchor.BN(0), {
      accounts: {
        user: user.publicKey,
        poll: pollPda,
        voter: voterPda,
        systemProgram: SystemProgram.programId,
      },
    });

    const voterAccount = await program.account.voter.fetch(voterPda);
    console.log('Voter Account:', voterAccount);
  });
});
