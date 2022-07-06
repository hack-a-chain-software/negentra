import create from 'zustand';
import { Contract, WalletConnection } from 'near-api-js';
import negentraNFT from '../../../contracts/testnet_settings/accounts/negentra_nft.testnet.json';
import negentraToken from '../../../contracts/testnet_settings/accounts/negentra_token.testnet.json';

export const useContract = create<{
  contractNFT,
  contractToken,
  isUpdating: boolean;
  initializeContracts: (
    walletConnection: WalletConnection,
    accountID: string
  ) => Promise<void>;
}>((set, get) => ({
  contractNFT: null,
  contractToken: null,
  isUpdating: false,

  initializeContracts: async (walletConnection: WalletConnection) => {
    set({ isUpdating: true });

    const contractNFT = new Contract(await walletConnection.account(), negentraNFT.account_id, {
      viewMethods: [
        'nft_burn',
        'update_item',
        'ft_on_transfer',
        'create_new_item',
        'change_mint_cost',
        'change_mint_token',
        'nft_transfer_payout',
        'change_perpetual_royalties',
      ],
      changeMethods: [],
    });

    const contractToken = new Contract(await walletConnection.account(), negentraToken.account_id, {
      viewMethods: [],
      changeMethods: [],
    });

    try {
      set({
        contractNFT,
        contractToken,
      });
      
    } finally {
      set({ isUpdating: false });
    }
  },
}));
