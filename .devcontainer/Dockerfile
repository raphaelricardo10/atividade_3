ARG CPLEX_INSTALLER_DIR=./cplex_installer
ARG CPLEX_INSTALLER_BIN=$CPLEX_INSTALLER_DIR/cplex_studio2210.linux_x86_64.bin
ARG CPLEX_INSTALLER_CONFIG=$CPLEX_INSTALLER_DIR/response.properties

FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye
WORKDIR /workspace
COPY . .

RUN touch test_file
# RUN chmod +x requirements.txt
# RUN chmod +x $CPLEX_INSTALLER_BIN
# RUN sudo apt update && sudo apt install -y pip default-jre
# RUN ./CPLEX_INSTALLER_BIN LAX_VM /usr/bin/java -f response.properties

# RUN /bin/sh -c "while sleep 1000; do :; done"