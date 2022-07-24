import { Tabs, TabList, TabPanels, Tab, TabPanel, Container, Flex, Image } from '@chakra-ui/react';
import items from '@negentra/public/json/play-section.json';
import { Text, Title } from '@negentra/src/components';

export function Play() {
  return (
    <Container
      id="play"
      marginBottom="180px"
      maxWidth="1410px"
      position="relative"
      className="flex justify-end px-0"
    >
      <Image
        src="/images/prism.png"
        position="absolute"
        bottom="-130px"
        left="-20px"
        className="hidden xl:block"
      />

      <Tabs
        w="100vw"
        maxW="1331px"
        align="center"
        variant="unstyled"
        orientation="vertical"
        paddingBottom="117px"
        paddingTop="88px"
        flexDirection="column-reverse"
        className="bg-[linear-gradient(110deg,_#9B59B6_60%,_#FF6F00_60%)] px-[18px] xl:px-0 xl:bg-[url(/images/play-bg.jpg)] xl:bg-[length:auto_100%] xl:bg-[center] xl:bg-no-repeat lg:h-[662px]"
      >
        <TabList
          bg="white"
          maxWidth="501px"
          marginX="auto"
          paddingY="8px"
          paddingX="9px"
          marginTop="46px"
          borderRadius="15px"
          display="flex"
          flexDirection="row"
          className="w-full"
        >
          {items && items.map(({ button }, i) => (
            <Flex
              key={'neg-play-list-' + i}
              flex="1"
            >
              <Tab
                h="62px"
                w="100%"
                bg="white"
                flexShrink="0"
                color="#FF6F00"
                borderRadius="12px"
                _selected={{ bg: '#9959B4', borderBottom: 'solid 5px #71368A', color: 'white' }}
              >
                  <Text
                    color="currentColor"
                    fontFamily="Titan One"
                  >
                    { button }
                  </Text>
              </Tab>
            </Flex>
          ))}
        </TabList>

        <TabPanels>
          {items && items.map(({ title, subtitle, description }, i) => (
            <TabPanel
              key={'neg-play-panel-' + i}
            >
              <Flex
                align="center"
                direction="column"
              >
                <Flex>
                  <Text
                    color="white"
                    fontSize="30px"
                    lineHeight="35px"
                    fontWeight="600"
                  >
                    { subtitle }
                  </Text>
                </Flex>

                <Flex
                  marginBottom="26px"
                >
                  <Title
                    fontSize="80px"
                    color="white"
                    lineHeight="94px"
                    className="text-[60px] leading-[1] sm:text-[80px] sm:leading-[94px]"
                    textShadow="0px 5px 0px #DDDDDD"
                  >
                    { title }
                  </Title>
                </Flex>

                <Flex
                  maxWidth="928px"
                  textAlign="center"
                  direction="column"
                  className="space-y-[21px]"
                >
                  {description && description?.map((text: string, i: number) => (
                    <Text
                      color="#FFFFFF"
                      key={'neg-play-desc' + i}
                    >
                      { text }
                    </Text>
                  ))}
                </Flex>
              </Flex>
            </TabPanel>
          ))}
        </TabPanels>
      </Tabs>
    </Container>
  );
};
