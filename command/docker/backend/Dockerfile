FROM eclipse-temurin:17-jdk-alpine

RUN apk update && apk add --no-cache bash

COPY ./app /server/
COPY ./web /server/web/

CMD ["/bin/bash", "/server/bin/app", "-Dconfig.file=/config/application.conf", "-Dlogger.file=/config/logback.xml"]

#./app/bin/app -Dconfig.file=config/application.conf  -Dlogger.file=config/logback.xml