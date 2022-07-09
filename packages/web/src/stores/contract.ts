import create from 'zustand';
import toast from 'react-hot-toast';
import { Contract, WalletConnection, utils } from 'near-api-js';

import baseContract from '@negentra/src/env/contract.json';

export const useContract = create<{
  contract;
  hasMinted: boolean
  initializeContract: (
    walletConnection: WalletConnection,
  ) => Promise<void>;
  mint: (
    type: string,
  ) => Promise<void>;
}>((set, get) => ({
  contract: null,
  hasMinted: false,

  initializeContract: async (walletConnection: WalletConnection) => {
    const account = await walletConnection.account();

    const contract = new Contract(account, baseContract.account_id, {
      viewMethods: [
        'nft_tokens_for_owner'
      ],
      changeMethods: [
        'nft_mint',
      ],
    }) as any;

    const mintedTokens = await contract?.nft_tokens_for_owner({
      account_id: account.accountId,
    });

    try {
      set({
        contract,
        hasMinted: mintedTokens?.length > 0,
      });
      
    } catch(e) {
      console.warn(e);
    }
  },

  mint: async (type: string) => {
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
