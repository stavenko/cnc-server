from debian:9

run apt-get update && \
    apt-get install libboost-all-dev gnupg apt-transport-https software-properties-common -y && \
    apt-key adv --keyserver hkp://keyserver.ubuntu.com --recv-key EF1B07FEE0EE663E && \
    apt-add-repository 'deb http://buildbot.linuxcnc.org/ stretch 2.8-rtpreempt' && \
    apt-get update && \
    apt-get -y install linuxcnc-uspace

entrypoint ["linuxcnc", "/config/basic-cnc.ini"]
    
  


