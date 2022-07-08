import { WalletConnection } from "near-api-js";
import { Image, Flex, Link, Button } from '@chakra-ui/react';
import { Text } from '@negentra/src/components';
import { useNavigate } from "react-router";
import { useCallback, useMemo, useEffect } from 'react';
import { useNearWallet, useNearUser } from 'react-near';
import { useContract } from '@negentra/src/stores/contract';

import contract from '../../../../contracts/testnet_settings/accounts/negentra_base_nft.testnet.json';

import menus from '@negentra/public/json/header.json';

export function Header() {
  const wallet = useNearWallet();
  const navigate = useNavigate();
  const user = useNearUser(contract.account_id);

  const {
    initializeContract,
  } = useContract();

  useEffect(() => {
    if (user.account && user.isConnected) {
      initializeContract(
        wallet as WalletConnection,
      );
    }
  }, [user.isConnected]);

  const login = useCallback(async () => {
    await wallet?.requestSignIn();
    await user.connect();
  }, [wallet]);
  
  const logout = useCallback(async () => {
    await wallet?.signOut();
    await user.disconnect();
  }, [wallet]);

  const walletConnected = useMemo(() => !!wallet?.isSignedIn(), [wallet]);

  return (
    <div className="w-screen flex items-center justify-center absolute">
      <nav
        className="w-full max-w-[1410px] h-[92px] mx-auto] flex items-center justify-center space-x-[18px]"
      >
        <Flex
          w='100%'
          h="100%"
          alignItems="center"
          p='0px 54px 12px 54px'
          justifyContent="space-between"
          className="rounded-b-[20px] bg-[url('/images/header-bg.png')] bg-[length:100%_100%]"
        >
          <Flex
            flexShrink="0"
            className="mx-auto md:mx-0"
            cursor="pointer"
            onClick={() => {navigate('/')}}
          >
            <Image src="/svg/logo.svg" h='52px' />
          </Flex>

          <Flex
            className="space-x-[16px] lg:space-x-[32px] hidden md:flex"
          >
            {menus.map((menu, i) =>
              <Flex
                key={'header-menu-item' + i}
              >
                <Link
                  isExternal={! menu.path?.startsWith('#')}
                  href={menu.path}
                  _hover={{
                    textDecoration: 'unset',
                  }}
                >
                  <Text
                    color='white'
                    fontSize='14px'
                    fontFamily="Titan One"
                    textTransform="uppercase"
                    _hover={{ color: '#EEEEEE' }}
                  >
                    { menu.name }
                  </Text>
                </Link>
              </Flex>
            )}
          </Flex>
        </Flex>

        <Flex
          h="100%"
          w="90px"
          alignItems="center"
          justifyContent="center"
          className="rounded-b-[19px] bg-[url('/images/wallet-bg.png')] bg-[length:100%_100%] pb-[12px]"
        >
          {
            walletConnected
            ? (
              <Button
                bg="transparent"
                onClick={() => logout()}
                _hover={{ bg: 'transparent' }}
              >
              <Image src="/svg/logout.svg" h='32px' />
              </Button>
            )
            : (
              <Button
                bg="transparent"
                onClick={() => login()}
                _hover={{ bg: 'transparent' }}
              >
                <Image src="/svg/wallet.svg" h='26px' />
              </Button>
            )
          }
        </Flex>
      </nav>
    </div>
  );
}
