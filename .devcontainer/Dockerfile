FROM --platform=linux/amd64 mcr.microsoft.com/devcontainers/rust:0-1-bullseye

ENV REMOTE_USER=vscode
ENV USER_HOME=/home/$REMOTE_USER
ENV WORKSPACE_FOLDER=/workspaces/atividade_3

ENV CPLEX_HOME=$USER_HOME/cplex
ENV CPLEX_INSTALLER_DIR=$USER_HOME/cplex_installer
ENV CPLEX_INSTALLER_BIN=$CPLEX_INSTALLER_DIR/cplex_studio2210.linux_x86_64.bin
ENV CPLEX_INSTALLER_CONFIG=$CPLEX_INSTALLER_DIR/response.properties

COPY cplex_installer $CPLEX_INSTALLER_DIR
RUN chmod +x $CPLEX_INSTALLER_BIN
RUN sudo apt update && sudo apt install -y pip default-jre python3-venv
RUN $CPLEX_INSTALLER_BIN LAX_VM /usr/bin/java -f $CPLEX_INSTALLER_CONFIG
RUN sudo rm -rdf $CPLEX_INSTALLER_DIR

ENV VENV_BIN=$WORKSPACE_FOLDER/venv/bin
ENV PYTHON=$VENV_BIN/python3
ENV PIP=$VENV_BIN/pip

COPY requirements.txt .
RUN sudo python3 $CPLEX_HOME/python/setup.py install
RUN python3 -m venv $WORKSPACE_FOLDER/venv
RUN $PIP install -r requirements.txt && rm -f requirements.txt

CMD ["sleep", "infinity"]