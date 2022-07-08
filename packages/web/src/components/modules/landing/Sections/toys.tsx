import Slider from 'react-slick';
import { useCallback, useRef } from 'react';
import { Button, Text, Title, Button3d } from '@negentra/src/components';
import { Container, Image, Flex } from '@chakra-ui/react';

import toys from '@negentra/public/json/toys.json';

const settings = {
  speed: 100,
  dots: false,
  arrows: false,
  infinite: true,
  slidesToShow: 7,
  centerMode: true,
  slidesToScroll: 1,
  responsive: [
    {
      breakpoint: 1024,
      settings: {
        slidesToShow: 5,
      }
    },
    {
      breakpoint: 640,
      settings: {
        slidesToShow: 1,
        infinite: false,
        centerMode: false,
      }
    },
  ],
  className: 'neg-skins-carousel bg-[url(/images/slider-bg.jpg)] h-[700px] bg-[length:706px_240px] bg-[center_bottom] bg-no-repeat items-center',
};

export function Toys() {
  const slider = useRef<Slider>();

  const onNext = useCallback(() => {
    slider.current?.slickNext();
  }, [slider.current]);

  const onPrev = useCallback(() => {
    slider.current?.slickPrev();
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
            className="text-[65px] leading-[1] sm:text-[100px] leading-[114.5px]"
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
        ref={slider as any}
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
              className="relative left-[60px] sm:left-0"
            />
          </Flex>
        ))}
      </Slider>

      <Flex
        width="100%"
        alignItems="center"
        justifyContent="center"
        className="space-x-[16px] mt-[32px] lg:space-x-[36px]"
      >
        <Button3d
          onClick={() => onPrev()}
          cursor="pointer"
          justifyContent="center"
          borderRadius="50%"
          color="#FF6F00"
          bg="#CA6110"
        >
          <Image
            w="24px"
            src="/svg/arrow-left.svg"
          />
        </Button3d>
        
        <Flex
          // transform="translateY(6px)"
        >
          <Button3d
            flex="1"
            bg="#CA6110"
            color="linear-gradient(180deg, #FC9F58 0%, #FF6F00 100%)"
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

              <Text
                color="white"
                fontSize="18px"
                fontFamily="Titan One"
                textShadow="0px 2px 0 #AAAAAA"
              >
                Mint Yours Now
              </Text>
            </Flex>
          </Button3d>
        </Flex>

        <Button3d
          onClick={() => onNext()}
          cursor="pointer"
          justifyContent="center"
          borderRadius="50%"
          color="#FF6F00"
          bg="#CA6110"
        >
          <Image
            w="24px"
            src="/svg/arrow-right.svg"
          />
        </Button3d>
      </Flex>
    </Container>
  );
};
