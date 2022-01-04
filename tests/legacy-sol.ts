import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { LegacySol } from '../target/types/legacy_sol';

const {SystemProgram} = anchor.web3;

describe('Setup legacy game', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  //@ts-ignore
  const program = anchor.workspace.LegacySol as Program<LegacySol>;
  const provider = anchor.Provider.env();


  it('Is initialized!', async () => {
    // Add your test here.
    const systemacc = await getPDA(provider.wallet.publicKey.toBuffer(), program.programId);
    const tx = await program.rpc.initialize(systemacc.bump, {
      accounts: {
        adminAccount: systemacc.account,
        admin: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId
      }
    });
    console.log("Initalization Admin Account:", await program.account.adminAccount.fetch(systemacc.account));
  });

  const gameId = "101";
  it("Only admin can create a game", async () => {
    const systemacc = await getPDA(provider.wallet.publicKey.toBuffer(), program.programId);
    const gameacc = await getPDA(Buffer.from(gameId), program.programId);
    //console.log("Initalization Admin Account:", await program.account.adminAccount.fetch(systemacc.account));
    const kp = anchor.web3.Keypair.generate();
    program.rpc.createGame(gameId, gameacc.bump, provider.wallet.publicKey, {
      accounts: {
        adminAccount: systemacc.account,
        admin: kp.publicKey,
        systemProgram: SystemProgram.programId,
        gameAccount: gameacc.account
      },
      signers: [kp]
    })
    .then(val => {throw new Error("Did not error correctly!");})
    .catch(err => {console.log("Errors Correctly!");})
  }) 

  it("Creates a game as admin", async () => {
    const systemacc = await getPDA(provider.wallet.publicKey.toBuffer(), program.programId);
    const gameacc = await getPDA(Buffer.from(gameId), program.programId);

    await program.rpc.createGame(gameId, gameacc.bump, provider.wallet.publicKey, {
      accounts: {
        adminAccount: systemacc.account,
        admin: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        gameAccount: gameacc.account
      }
    })
    console.log(await program.account.gameAccount.fetch(gameacc.account));    
  })
});

const getPDA = async (accBuf: Buffer, programId: anchor.web3.PublicKey) => {
  const [acc, accbmp] = await anchor.web3.PublicKey.findProgramAddress([accBuf], programId)
  return {account: acc, bump: accbmp};
}