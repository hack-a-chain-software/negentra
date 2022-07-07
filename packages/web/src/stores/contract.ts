import create from 'zustand';
import { Contract, WalletConnection, utils } from 'near-api-js';

import baseContract from '../../../contracts/testnet_settings/accounts/negentra_base_nft.testnet.json';

export const useContract = create<{
  contract: Contract | null;
  initializeContract: (
    walletConnection: WalletConnection,
  ) => Promise<void>;
  mint: (
    type: string,
  ) => Promise<void>;
}>((set, get) => ({
  contract: null,

  initializeContract: async (walletConnection: WalletConnection) => {
    const contract = new Contract(await walletConnection.account(), baseContract.account_id, {
      viewMethods: [
        //  
      ],
      changeMethods: [
        'nft_mint',
      ],
    });

    try {
      set({
        contract,
      });
      
    } catch(e) {
      console.warn(e);
    }
  },

  mint: async (type: string = 'Male') => {
    const amountInYocto = utils.format.parseNearAmount('1');

    await get().contract?.nft_mint({
      args: {
        user_type: {
          type,
        },
      },
      amount: amountInYocto,
    });

    console.log('aguardando.......');
  },
}));
