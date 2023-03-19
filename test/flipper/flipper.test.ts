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
      (await flipperFactory.new()).address,
      deployer,
      api
    )
  })

  after(async function tearDown() {
    await api.disconnect()
  })

  it("Sets the owner",async () => {
    expect((await contract.query.owner()).value.ok).to.equal(deployer.address)
  })    

  it("Can flip the state with emiting an event", async () => {
    const { gasRequired } = await contract.withSigner(deployer).query.flip("")
    await contract.withSigner(signer).tx.flip("",{
      gasLimit: gasRequired,
    })
    
    const flipped = await new Promise<Flipped>((resolve) => {
      contract.events.subscribeOnFlippedEvent(async (e) => {   
        console.log(e)                      
        resolve(e)
      })
    })        
    
    expect(signer.address).to.equal(flipped.caller)
    
  })
})
