FROM --platform=linux/amd64 mcr.microsoft.com/devcontainers/rust:0-1-bullseye

ENV CPLEX_INSTALLER_DIR=/home/vscode/cplex_installer
ENV CPLEX_INSTALLER_BIN=$CPLEX_INSTALLER_DIR/cplex_studio2210.linux_x86_64.bin
ENV CPLEX_INSTALLER_CONFIG=$CPLEX_INSTALLER_DIR/response.properties

COPY cplex_installer $CPLEX_INSTALLER_DIR

RUN chmod +x $CPLEX_INSTALLER_BIN
RUN sudo apt update && sudo apt install -y pip default-jre
RUN $CPLEX_INSTALLER_BIN LAX_VM /usr/bin/java -f $CPLEX_INSTALLER_CONFIG
RUN sudo rm -rdf $CPLEX_INSTALLER_DIR