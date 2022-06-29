import { Tabs, TabList, TabPanels, Tab, TabPanel, Container, Flex, Text, Grid, Image } from '@chakra-ui/react'

const items = [
  {
    title: 'What is Sociapol ?',
    image: '/images/about1.jpeg',
    description: (
      <Flex
        maxWidth="570px"
        direction="column"
        className="space-y-[21px]"
      >
        <Text
          fontSize="18px"
          color="#333333"
          lineHeight="21px"
        >
           Negentra ekibinin geliştirmiş olduğu Sociapol, size sanal bir dünyanın kapılarını aralıyor. Bu sanal dünya üzerinde çok farklı eğlence araçları ve karakterler bulunuyor. Sanal dünya üzerinde yapacağın etkileşimlerini, diğer insanlar ile beraber gerçekleştir.
        </Text>
        
        <Text
          fontSize="18px"
          color="#333333"
          lineHeight="21px"
        >
          Sürekli gelişen ve yapısal olarak kendini güncelleyen Sociapol, yenilikleri ile beraber sınırsız eğlenceyi size sunuyor. Futbol oyna, aksiyon eğlencelerine katıl, partilerin üyesi ol. Veya kripto alt yapısı ile alışverişlerini gerçekleştir. Bunların hepsi Sociapol ile mümkün.
        </Text>
      </Flex>
    )
  },
  {
    title: 'What is Sociapol ?',
    image: '/images/about1.jpeg',
    description: (
      <Flex
        direction="column"
        className="space-y-[21px]"
      >
        <Text
          fontSize="18px"
          color="#333333"
          lineHeight="21px"
        >
           Negentra ekibinin geliştirmiş olduğu Sociapol, size sanal bir dünyanın kapılarını aralıyor. Bu sanal dünya üzerinde çok farklı eğlence araçları ve karakterler bulunuyor. Sanal dünya üzerinde yapacağın etkileşimlerini, diğer insanlar ile beraber gerçekleştir.
        </Text>
        
        <Text
          fontSize="18px"
          color="#333333"
          lineHeight="21px"
        >
          Sürekli gelişen ve yapısal olarak kendini güncelleyen Sociapol, yenilikleri ile beraber sınırsız eğlenceyi size sunuyor. Futbol oyna, aksiyon eğlencelerine katıl, partilerin üyesi ol. Veya kripto alt yapısı ile alışverişlerini gerçekleştir. Bunların hepsi Sociapol ile mümkün.
        </Text>
      </Flex>
    )
  },
  {
    title: 'What is Sociapol ?',
    image: '/images/about1.jpeg',
    description: (
      <Flex
        direction="column"
        className="space-y-[21px]"
      >
        <Text
          fontSize="18px"
          color="#333333"
          lineHeight="21px"
        >
           Negentra ekibinin geliştirmiş olduğu Sociapol, size sanal bir dünyanın kapılarını aralıyor. Bu sanal dünya üzerinde çok farklı eğlence araçları ve karakterler bulunuyor. Sanal dünya üzerinde yapacağın etkileşimlerini, diğer insanlar ile beraber gerçekleştir.
        </Text>
        
        <Text
          fontSize="18px"
          color="#333333"
          lineHeight="21px"
        >
          Sürekli gelişen ve yapısal olarak kendini güncelleyen Sociapol, yenilikleri ile beraber sınırsız eğlenceyi size sunuyor. Futbol oyna, aksiyon eğlencelerine katıl, partilerin üyesi ol. Veya kripto alt yapısı ile alışverişlerini gerçekleştir. Bunların hepsi Sociapol ile mümkün.
        </Text>
      </Flex>
    )
  },
  {
    title: 'What is Sociapol ?',
    image: '/images/about1.jpeg',
    description: (
      <Flex
        direction="column"
        className="space-y-[21px]"
      >
        <Text
          fontSize="18px"
          color="#333333"
          lineHeight="21px"
        >
           Negentra ekibinin geliştirmiş olduğu Sociapol, size sanal bir dünyanın kapılarını aralıyor. Bu sanal dünya üzerinde çok farklı eğlence araçları ve karakterler bulunuyor. Sanal dünya üzerinde yapacağın etkileşimlerini, diğer insanlar ile beraber gerçekleştir.
        </Text>
        
        <Text
          fontSize="18px"
          color="#333333"
          lineHeight="21px"
        >
          Sürekli gelişen ve yapısal olarak kendini güncelleyen Sociapol, yenilikleri ile beraber sınırsız eğlenceyi size sunuyor. Futbol oyna, aksiyon eğlencelerine katıl, partilerin üyesi ol. Veya kripto alt yapısı ile alışverişlerini gerçekleştir. Bunların hepsi Sociapol ile mümkün.
        </Text>
      </Flex>
    )
  },
];

export function About() {
  return (
    <Container
      h="100%"
      w="100vw"
      maxWidth="1410px"
      className="flex items-center"
    >
      <Tabs
        w="100%"
        align="center"
        variant="unstyled"
        orientation="vertical"
        flexDirection="row-reverse"
      >
        <TabList>
          {[...Array(items.length)].map((_, i) => (
            <Flex
              key={i}
              align="start"
              position="relative"
              direction="column"
              alignItems="center"
              justifyContent="center"
            >
              { i > 0 && 
                <Flex
                  h="46px"
                  w="4px"
                  marginY="13px"
                  flexShrink="0"
                  background="#9959B4"
                  borderRadius="1.5px"
                /> 
              }
              
              <Tab
                w="64px"
                h="64px"
                bg="white"
                flexShrink="0"
                borderRadius="50%"
                border="1px solid #9959B4"
                color="#9959B4"
                _selected={{ bg: '#9959B4', borderBottom: 'solid 4px #71368A', color: 'white' }}
              >
                  <Text
                    fontSize="18px"
                    fontWeight="500"
                  >
                    { i + 1 }
                  </Text>
              </Tab>
            </Flex>
          ))}
        </TabList>

        <TabPanels>
          {items.map(({ title, description, image }, i) => (
            <TabPanel
              key={i}
            >
              <Grid
                templateColumns="auto 546px"
              >
                <Flex
                  direction="column"
                >
                  <Flex
                    maxW="385px"
                    marginBottom="26px"
                  >
                    <Text
                      fontSize="80px"
                      color="#333333"
                      lineHeight="94px"
                      textAlign="start"
                    >
                      { title }
                    </Text>
                  </Flex>

                  <Flex
                    maxWidth="570px"
                    textAlign="start"
                  >
                    { description }
                  </Flex>
                </Flex>

                <Image
                  src={image}
                />
              </Grid>
            </TabPanel>
          ))}
        </TabPanels>
      </Tabs>
    </Container>
  );
};