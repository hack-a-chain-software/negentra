import { useNavigate } from 'react-router';
import { WalletConnection } from 'near-api-js';
import { Image, Flex, Link } from '@chakra-ui/react';
import { Text, Button3d } from '@negentra/src/components';
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
    checkMinted,
    initializeContract,
  } = useContract();

  useEffect(() => {
    if (user.account && user.isConnected) {
      initializeContract(
        wallet as WalletConnection,
      ).then(() => {
        checkMinted(user.address);
      });
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
            className="space-x-[16px] lg:space-x-[32px] hidden md:flex items-center"
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

            <Flex
              transform="translateY(3px)"
            >
              {
                walletConnected
                ? (
                  <Button3d
                    bg="#EEEEEE"
                    color="white"
                    minHeight="auto"
                    padding="8px 18px"
                    borderRadius="12px"
                    onClick={() => logout()}
                  >
                    <Flex>
                      <Text
                        color="#FF6F00"
                        fontSize="14px"
                        marginRight="12px"
                        fontFamily="Titan One"
                      >
                        { user.address?.split('.').at(0) }...
                      </Text>

                      <Image src="/svg/logout.svg" h='24px' />
                    </Flex>
                  </Button3d>
                )
                : (
                  <Button3d
                    bg="#EEEEEE"
                    color="white"
                    minHeight="auto"
                    padding="8px 18px"
                    borderRadius="12px"
                    onClick={() => login()}
                  >
                    <Flex>
                      <Text
                        color="#FF6F00"
                        fontSize="14px"
                        marginRight="12px"
                        fontFamily="Titan One"
                      >
                        Connect Wallet
                      </Text>

                      <Image src="/svg/wallet.svg" h='24px' />
                    </Flex>
                  </Button3d>
                )
              }
            </Flex>
          </Flex>
          </Flex>
      </nav>
    </div>
  );
}
