import { expect, use } from "chai"
import chaiAsPromised from "chai-as-promised"
import FlipperFactory from "./typedContract/constructors/flipper"
import Flipper from "./typedContract/contracts/flipper"
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api"
import { KeyringPair } from "@polkadot/keyring/types"
import { Flipped } from "./typedContract/event-types/flipper"

use(chaiAsPromised)

// Create a new instance of contract
const wsProvider = new WsProvider("ws://127.0.0.1:9944")
// Create a keyring instance
const keyring = new Keyring({ type: "sr25519" })

describe("flipper test", () => {
  let flipperFactory: FlipperFactory
  let api: ApiPromise
  let deployer: KeyringPair
  let signer: KeyringPair

  let contract: Flipper
  const initialState = true

  before(async function setup(): Promise<void> {
    api = await ApiPromise.create({ provider: wsProvider })
    deployer = keyring.addFromUri("//Alice")
    signer = keyring.addFromUri("//Bob")

    flipperFactory = new FlipperFactory(api, deployer)

    contract = new Flipper(
      (await flipperFactory.new("first_pass")).address,
      deployer,
      api
    )
  })

  after(async function tearDown() {
    await api.disconnect()
  })

  it("Can flip the state with emiting an event and next signer can flip", async () => {
    const first_pass = "first_pass"
    const second_pass = "second_pass"
    const third_pass = "third_pass"

    const { gasRequired } = await contract.withSigner(deployer).query.flip(first_pass, second_pass)
    await contract.withSigner(deployer).tx.flip(first_pass, second_pass ,{
      gasLimit: gasRequired,
    })
    
    let flipped = await new Promise<Flipped>((resolve) => {
      contract.events.subscribeOnFlippedEvent(async (e) => {                              
        resolve(e)
      })
    }) 

    expect(flipped.caller).to.equal(deployer.address)
    expect(flipped.nextPass).to.equal(second_pass)
        
    await contract.withSigner(signer).tx.flip(second_pass, third_pass,{
      gasLimit: gasRequired,
    })

    flipped = await new Promise<Flipped>((resolve) => {
      contract.events.subscribeOnFlippedEvent(async (e) => {                              
        resolve(e)
      })
    }) 

    expect(flipped.caller).to.equal(signer.address)
    expect(flipped.nextPass).to.equal(third_pass)
  })
})
