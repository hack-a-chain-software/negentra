import Slider from 'react-slick';
import { useCallback, useRef } from 'react';
import { Button, Text, Title } from '@negentra/src/components';
import { Container, Image, Flex } from '@chakra-ui/react';

import toys from '@negentra/public/json/toys.json';

export function Toys() {
  const slider = useRef<typeof Slider>();

  const settings = {
    speed: 500,
    dots: false,
    infinite: true,
    slidesToShow: 7,
    centerMode: true,
    slidesToScroll: 1,
    className: 'neg-skins-carousel bg-[url(/images/slider-bg.jpg)] h-[700px] bg-[length:706px_240px] bg-[center_bottom] bg-no-repeat items-center',
  };

  const onNext = useCallback(() => {
    slider.current.slickNext();
  }, [slider.current]);

  const onPrev = useCallback(() => {
    slider.current.slickPrev();
  }, [slider.current]);

  return (
    <Container
      h="100%"
      w="100vw"
      minHeight="100vh" 
      maxWidth="1920px"
      marginBottom="152px"
    >
      <Container
        w="100vw"
        maxW="1410px"
        marginBottom="64px"
      >
        <Flex
          marginBottom="12px"
          justifyContent="center"
        >
          <Title
            textTransform="uppercase"
          >
            Characters
          </Title>
        </Flex>

        <Flex>
          <Text
            textAlign="center"
          >
            Sociapol’e üye olduğunda ilk yapacağın şey karakterini yaratmak olacak. Karakterinin giyeceği bütün kıyafet ve aksesuarlar ise birer NFT olarak tasarlandı – dolayısıyla sana ve karakterine ait olacak. Bu NFT’leri satabilir, takas edebilir, yılın belirli dönemlerine özel çıkaracağımız koleksiyonlardan satın alabilirsin. Karakterinin nasıl görüneceği tamamen sana bağlı, ve sana ait!
          </Text>
        </Flex>
      </Container>

      <Slider
        {...settings}
        ref={slider}
      >
        {toys && toys.map((toy: string, i: number) => (
          <Flex
            key={'neg-slider-toy' + i}
            display="flex"
            alignItems="center"
            justifyContent="center"
            style={{
              width: '231px',
            }}
          >
            <Image
              w="231px"
              src={toy}
            />
          </Flex>
        ))}
      </Slider>

      <Flex
        width="100%"
        alignItems="center"
        justifyContent="center"
        className="space-x-[36px] mt-[32px]"
      >
        <Flex
          onClick={() => onPrev()}
          align="center"
          cursor="pointer"
          justifyContent="center"
          h="68px"
          w="100px"
          borderRadius="50%"
          className="button-3d"
          style={{
            '--primary': '#FF6F00',
            '--secondary': '#CA6110',
          }}
        >
          <Image
            h="24px"
            w="24px"
            src="/svg/arrow-left.svg"
          />
        </Flex>
        
        <Flex
          transform="translateY(6px)"
        >
          <Button
            flex="1"
            bg="linear-gradient(180deg, #FC9F58 0%, #FF6F00 100%)"
            borderBottom="solid 8px #CA6110"
          >
            <Flex
              alignItems="center"
              justifyContent="center"
            >
              <Flex
                w="34px"
                h="34px"
                bg="white"
                marginRight="15px"
                borderRadius="50%"
                alignItems="center"
                justifyContent="center"
              >
                <Image
                  src="/svg/bag.svg"
                  h="16px"
                  w="16px"
                />
              </Flex>

              Mint Yours Now
            </Flex>
          </Button>
        </Flex>

        <Flex
          onClick={() => onNext()}
          align="center"
          cursor="pointer"
          justifyContent="center"
          className="button-3d"
          h="68px"
          w="100px"
          borderRadius="50%"
          style={{
            '--primary': '#FF6F00',
            '--secondary': '#CA6110',
          }}
        >
          <Image
            h="24px"
            w="24px"
            src="/svg/arrow-right.svg"
          />
        </Flex>
      </Flex>
    </Container>
  );
};
