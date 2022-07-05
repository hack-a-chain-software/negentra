import create from "zustand";
import { GreetingContract } from "@near/contracts";
import { Contract, WalletConnection } from "near-api-js";
import { contractName } from "../env/contract";

export const useContract = create<{
  contract: GreetingContract | null;
  isUpdating: boolean;
  initializeContract: (
    walletConnection: WalletConnection,
    accountID: string
  ) => Promise<void>;
}>((set, get) => ({
  contract: null,
  isUpdating: false,

  initializeContract: async (walletConnection: WalletConnection) => {
    set({ isUpdating: true });
    const contract = new GreetingContract(
      new Contract(await walletConnection.account(), contractName, {
        // View methods are read only. They don't modify the state, but usually return some value.
        viewMethods: ["get_greeting"],
        // Change methods can modify the state. But you don't receive the returned value when called.
        changeMethods: ["set_greeting"],
      })
    );

    try {
      set({
        contract,
      });
      
    } finally {
      set({ isUpdating: false });
    }
  },
}));
