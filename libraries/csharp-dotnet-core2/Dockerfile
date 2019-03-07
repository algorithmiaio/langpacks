RUN apt-get -y update && \
    apt-get install --no-install-recommends -y \
        build-essential \
        ca-certificates \
        libssl-dev \
        wget

# Install .NET Core SDK
ENV DOTNET_SDK_VERSION 2.2
ENV UBUNTU_VERSION 16.04

RUN wget -q https://packages.microsoft.com/config/ubuntu/$UBUNTU_VERSION/packages-microsoft-prod.deb
RUN dpkg -i packages-microsoft-prod.deb

RUN apt-get install -y apt-transport-https && \
    apt-get update && \
    apt-get install -y dotnet-sdk-$DOTNET_SDK_VERSION

RUN dotnet help