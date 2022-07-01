import Slider from 'react-slick';
import { Title } from '@negentra/src/components';
import { Container, Flex, Image } from '@chakra-ui/react';

import partners from '@negentra/public/json/partners.json';

export function Partners() {
  const settings = {
    rows: 2,
    infinite: true,
    slidesToShow: 4,
    slidesPerRow: 1,
  };

  return (
    <Container
      w="100vw"
      minHeight="1124px"
      maxWidth="1920px"
      paddingTop="150px"
      marginBottom="40px"
      className="bg-[url(/svg/partners-bg.svg)] bg-[length:1920px_1007px] bg-no-repeat"
    >
      <Container
        w="100vw"
        maxW="1410px"
      >
        <Flex
          w="100%"
          mx="auto"
          alignItems="center"
          justifyContent="center"
        >
          <Flex>
            <Title
              color="white"
            >
              Partners
            </Title>
          </Flex>
        </Flex>

        <Slider
            {...settings}
            padding="10px 48px"
          >
            {partners && partners.map((logo: string, i: number) => (
              <Container
                key={i}
                paddingY="12px"
                className="flex items-center justify-center"
              >
                <Flex
                  h="196px"
                  background="white"
                  borderRadius="19px"
                  alignItems="center"
                  justifyContent="center"
                >
                  <Image
                    src={logo}
                    h="52px"
                  />
                </Flex>
              </Container>
            ))}
          </Slider>
      </Container>
    </Container>
  );
};