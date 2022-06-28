import { Image, Flex, Text, Link, Button } from "@chakra-ui/react";

const menuItems = [
  {
    name: 'Sociapol',
    path: '#',
  },
  {
    name: 'NFT',
    path: '#',
  },
  {
    name: 'SPOL',
    path: '#',
  },
  {
    name: 'Roadmap',
    path: '#',
  },
  {
    name: 'Crew',
    path: '#',
  },
  {
    name: 'Partners',
    path: '#',
  },
  {
    name: 'Whitepaper',
    path: '#',
  },
];

export function Header() {
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
          <Flex>
            <Image src="/svg/logo.svg" h='52px' />
          </Flex>

          <Flex
            className="space-x-[32px]"
          >
            {menuItems.map((menu, i) =>
              <Flex
                key={i}
              >
                <Link
                  isExternal
                  href={menu.path}
                >
                  <Text
                    color='#EEEEEE'
                    fontSize='14px'
                    className="uppercase"
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
          <Button
            bg="transparent"
            _hover={{ bg: 'transparent' }}
          >
            <Image src="/svg/wallet.svg" h='26px' />
          </Button>
        </Flex>
      </nav>
    </div>
  );
}
