import { Container, Image, Flex, Grid, GridItem, Text, Link, Box } from "@chakra-ui/react";

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

const menuLinks = [
  {
    name: 'Privacy Policy',
    path: '#',
  },
  {
    name: 'Cookie Policy',
    path: '#',
  },
  {
    name: 'Clarification Policy',
    path: '#',
  },
];

const socialLinks = [
  {
    path: '#',
    icon: 'facebook',
  },
  {
    path: '#',
    icon: 'twitter',
  },
  {
    path: '#',
    icon: 'instagram',
  },
  {
    path: '#',
    icon: 'discord',
  },
];

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
                  >
                    <Text
                      color='#EEEEEE'
                      fontSize='18px'
                      lineHeight="24px"
                      className="uppercase"
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
              {menuLinks.map((menu, i) =>
                <Flex
                  key={i}
                >
                  <Link
                    isExternal
                    href={menu.path}
                  >
                    <Text
                      color='#EEEEEE'
                      fontSize='18px'
                      lineHeight="24px"
                      className="uppercase"
                    >
                      { menu.name }
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
                  fontSize="18px"
                >
                  Follow Us
                </Text>
              </Flex>

              <Flex
                className="space-x-[10px]"
              >   
                {socialLinks.map((menu, i) =>
                  <Link
                    isExternal
                    href={menu.path}
                    key={i}
                  >
                    <Flex
                      w="42px"
                      h="42px"
                      alignItems="center"
                      borderRadius="50%"
                      background="#71368A"
                      justifyContent="center"
                    >
                        <Image
                          src={`/svg/${menu.icon}.svg`}
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
