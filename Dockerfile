FROM alpine

RUN mkdir /study

COPY ./target/release/auto-youth-learning /study/auto-youth-learning

WORKDIR /study

RUN chmod +x auto-youth-learning

CMD ["./auto-youth-learning"]