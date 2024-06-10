import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Crosschain } from "../target/types/crosschain";

import {ethers, hexlify, toUtf8Bytes} from "ethers";

describe("crosschain", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Crosschain as Program<Crosschain>;



  it("send packet",async ()=>{

    const tx = await  program.methods.send().rpc();
    console.log("Your transaction signature", tx);
  })

  it("receiving packet",async ()=>{


    //const tx = await  program.methods.send().rpc();
    console.log("Your transaction signature");
  })
});
