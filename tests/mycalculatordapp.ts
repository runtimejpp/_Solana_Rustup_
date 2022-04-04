const assert = require('assert')
const anchor = require('@project-serum/anchor')
const {SystemProgram} = anchor.web3
describe('mycalculatordapp',() =>{
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);
  const calculator = anchor.web3.Keypair.generate();
  const program = anchor.workspace.Mycalculatordapp

  it('Creates a Calculator', async()=>  {
    await program.rpc.create('Solana Welcomes you - Test Successful', {
      accounts:{
        calculator: calculator.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId


      },
    })
    const account = await program.account.calculator.fetch(calculator.publicKey); 
    assert.ok(account.greeting === " Solana welcomes you! ")
  })
})  