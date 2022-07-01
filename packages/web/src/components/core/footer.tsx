import { Container, Image, Flex, Grid, Link } from "@chakra-ui/react";
import { Text } from '@negentra/src/components';
import menuItems from '@negentra/public/json/header.json';
import menuLinks from '@negentra/public/json/policy.json';
import socialLinks from '@negentra/public/json/social.json';

export function Footer() {  
  return (
    <Container
      w="100vw"
      maxW="1920px"
      className="bg-[url('/images/footer-bg.png')] bg-[length:100%_530px] bg-bottom bg-no-repeat h-[640px]"
    >
      <Container
        w="100%"
        h="100%"
        maxW="1410px"
      >
        <Grid
          templateColumns="585px auto"
        >
          <Flex
            w="100%"
          >
            <Image
              src="/images/footer-toy.png"
              h="100%"
            />
          </Flex>

          <Flex
            w="100%"
            h="400px"
            marginLeft="-85px"
            className="mt-auto space-x-[135px] pt-[66px]"
          >
            <Flex
              direction="column"
              className="space-y-[4px]"
            >
              {menuItems.map((menu, i) =>
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

            <Flex
              direction="column"
              className="space-y-[4px]"
            >
              {menuLinks.map((policy, i) =>
                <Flex
                  key={i}
                >
                  <Link
                    isExternal
                    href={policy.path}
                    _hover={{
                      textDecoration: 'unset',
                    }}
                  >
                    <Text
                      color='white'
                      fontFamily="Titan One"
                      textTransform="uppercase"
                      _hover={{ color: '#EEEEEE' }}
                    >
                      { policy.name }
                    </Text>
                  </Link>
                </Flex>
              )}
            </Flex>

            <Flex
              direction="column"
            >
              <Flex
                marginBottom="12px"
              >
                <Text
                  color="white"
                  fontFamily="Titan One"
                >
                  Follow Us
                </Text>
              </Flex>

              <Flex
                className="space-x-[10px]"
              >   
                {socialLinks.map((social, i) =>
                  <Link
                    isExternal
                    href={social.path}
                    key={i}
                    _hover={{
                      textDecoration: 'unset',
                    }}
                  >
                    <Flex
                      w="42px"
                      h="42px"
                      alignItems="center"
                      borderRadius="50%"
                      background="#71368A"
                      justifyContent="center"
                      className="hover:bg-[#71368A]/80"
                    >
                      <Image
                        src={`/svg/${social.icon}.svg`}
                      />
                    </Flex>
                  </Link>
                )}
              </Flex>
            </Flex>
          </Flex>
        </Grid>
      </Container>
    </Container>
  );
}
