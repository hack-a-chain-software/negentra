import { Container, Image, Flex, Grid, Link } from "@chakra-ui/react";
import { Text } from '@negentra/src/components';
import menuItems from '@negentra/public/json/header.json';
import policyItems from '@negentra/public/json/policy.json';
import socialLinks from '@negentra/public/json/social.json';

export function Footer() {  
  return (
    <Container
      maxW="1920px"
      className="bg-[url('/images/footer-bg.png')] bg-[length:100%_530px] bg-bottom bg-no-repeat xl:h-[640px]"
    >
      <Container
        maxW="1410px"
      >
        <Grid
          templateColumns="585px auto"
          className="xl:grid"
        >
          <Flex
            w="100%"
            className="hidden xl:block"
          >
            <Image
              src="/images/footer-toy.png"
              h="100%"
            />
          </Flex>

          <Flex
            w="100%"
            minH="400px"
            className="mt-auto sm:space-x-[135px] pt-[66px] xl:ml-[-85px] space-y-[18px] flex-col sm:flex-row"
          >
            <Flex
              direction="column"
              className="space-y-[4px]"
            >
              {menuItems.map((menu, i) =>
                <Flex
                  key={'footer-menu-item' + i}
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
              className="space-y-[4px] hidden md:block"
            >
              {policyItems.map((policy, i) =>
                <Flex
                  key={'footer-policy-item' + i}
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
                    key={'footer-social-item' + i}
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
