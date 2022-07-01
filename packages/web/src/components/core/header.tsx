import { Image, Flex, Link } from '@chakra-ui/react';
import { Text } from '@negentra/src/components';

import menus from '@negentra/public/json/header.json';

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
          <Flex
            flexShrink="0"
          >
            <Image src="/svg/logo.svg" h='52px' />
          </Flex>

          <Flex
            className="space-x-[16px] lg:space-x-[32px] hidden md:flex"
          >
            {menus.map((menu, i) =>
              <Flex
                key={i}
              >
                <Link
                  isExternal
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
      </nav>
    </div>
  );
}
