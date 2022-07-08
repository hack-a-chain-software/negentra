import create from 'zustand';
import toast from 'react-hot-toast';
import { Contract, WalletConnection, utils } from 'near-api-js';

import baseContract from '../../../contracts/testnet_settings/accounts/negentra_base_nft.testnet.json';

export const useContract = create<{
  contract;
  mintedChar: boolean
  initializeContract: (
    walletConnection: WalletConnection,
  ) => Promise<void>;
  mint: (
    type: string,
  ) => Promise<void>;
  checkMinted: (
    accountId: string
  ) => Promise<void>;
}>((set, get) => ({
  contract: null,
  mintedChar: false,

  initializeContract: async (walletConnection: WalletConnection) => {
    const contract = new Contract(await walletConnection.account(), baseContract.account_id, {
      viewMethods: [
        'nft_tokens_for_owner'
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

  checkMinted: async (accountId: string) => {
    const tokens = await get().contract?.nft_tokens_for_owner({
      account_id: accountId,
    });

    try {
      set({
        mintedChar: tokens && tokens.length > 0,
      });
      
    } catch(e) {
      console.warn(e);
    }
  },

  mint: async (type: string | undefined) => {
    if (!type) {
      toast("Select character's gender to mint");

      return;
    }

    await get().contract?.nft_mint({
      args: {
        user_type: {
          type,
        },
      },
      amount: utils.format.parseNearAmount('0.5'),
    });

    console.log('aguardando.......');
  },
}));
