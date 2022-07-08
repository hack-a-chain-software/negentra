import Slider from 'react-slick';
import { useCallback, useRef } from 'react';
import { Text, Title, Button3d } from '@negentra/src/components';
import { Container, Flex, Image } from '@chakra-ui/react';

import roadmap from '@negentra/public/json/roadmap.json';

export function Roadmap() {
  const slider = useRef<Slider>();

  const onNext = useCallback(() => {
    slider.current?.slickNext();
  }, [slider.current]);

  const onPrev = useCallback(() => {
    slider.current?.slickPrev();
  }, [slider.current]);

  const settings = {
    speed: 100,
    dots: true,
    infinite: true,
    arrows: false,
    slidesToShow: 3,
    centerMode: true,
    slidesToScroll: 1,
    responsive: [
      {
        breakpoint: 1280,
        settings: {
          slidesToShow: 1,
        }
      },
    ],
    centerPadding: '0px 80px',
    className: 'neg-roadmap-carousel',
    customPaging: i => (
      <div
        className="w-[18px] h-[8px] rounded-[12px] bg-white hover:bg-white/75"
      />
    ),
    appendDots: dots => (
      <div
        className="flex flex-row"
        style={{
          display: 'flex',
          flexDirection: 'row',
          alignItems: 'center',
          justifyContent: 'center',
        }}
      >
        <Button3d
          bg="#EEEEEE"
          color="white"
          width="64px"
          borderRadius="50%"
          marginRight="16px"
          padding="16px 0px"
          onClick={() => onPrev()}
        >
          <Image
            w="28px"
            src="/svg/lg-arrow-left.svg"
            className="mx-auto"
          />
        </Button3d>
  
        <ul style={{ margin: "0px", height: '8px' }}> {dots} </ul>
  
        <Button3d
          bg="#EEEEEE"
          color="white"
          width="64px"
          marginLeft="16px"
          borderRadius="50%"
          padding="16px 0px"
          onClick={() => onNext()}
        >
          <Image
            w="28px"
            src="/svg/lg-arrow-right.svg"
            className="mx-auto"
          />
        </Button3d>
      </div>
    ),
  };

  return (
    <Container
      id="roadmap"
      minHeight="1124px"
      maxWidth="1920px"
      paddingTop="150px"
      marginBottom="136px"
      className="bg-[url(/svg/roadmap-bg.svg)] bg-[length:1920px_1124px] bg-no-repeat"
    >
      <Container
        w="100vw"
        maxW="1440px"
      >
        <Flex
          w="100%"
          mx="auto"
          alignItems="center"
          justifyContent="center"
          marginBottom="22px"
        >
          <Flex>
            <Title
              color="white"
              fontSize="100px"
              textTransform="uppercase"
              className="text-[60px] leading-[1] sm:text-[100px] leading-[114.5px]"
            >
              Roadmap
            </Title>
          </Flex>
        </Flex>

        <Slider
          {...settings}
          ref={slider as any}
        >
          { roadmap && roadmap.map(({ title, description }, i) => (
            <Container
              key={'neg-roadmap-item' + i}
              className="flex items-center justify-center h-[450px]"
            >
              <Flex
                width="392px"
                color="white"
                display="flex"
                paddingX="24px"
                minHeight="342px"
                direction="column"
                alignItems="center"
                paddingBottom="44px"
                justifyContent="center"
                className="reg-roadmap-carousel-slide bg-[url(/svg/roadmap-border.svg)] bg-[length:100%_100%] bg-no-repeat"
              >
                <Text
                  fontSize="30px"
                  textAlign="center"
                  color="currentColor"
                  marginBottom="18px"
                  fontFamily="Titan One"
                  className="neg-roadmap__title"
                >
                  { title }
                </Text>

                <Text
                  fontSize="14px"
                  color="currentColor"
                  fontWeight="400"
                  textAlign="center"
                  className="neg-roadmap__text"
                >
                  { description }
                </Text>
              </Flex>
            </Container>
          ))}
        </Slider>
      </Container>
    </Container>
  );
};