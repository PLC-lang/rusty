FROM mcr.microsoft.com/windows/servercore:ltsc2019

ARG LLVM_VER=11.0.1
ARG RUST_VER=1.54.0

# Install VC++

WORKDIR C:/buildTools

# Download channel for fixed install.
ARG CHANNEL_URL=https://aka.ms/vs/16/release/channel
ADD ${CHANNEL_URL} C:/TEMP/VisualStudio.chman

ADD https://aka.ms/vs/16/release/vs_buildtools.exe C:/TEMP/vs_buildtools.exe
RUN C:/TEMP/vs_buildtools.exe --quiet --wait --norestart --nocache --installPath C:/buildTools \
				# --all
				--add Microsoft.Component.MSBuild \
				--add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 \
				--add Microsoft.VisualStudio.Component.Windows10SDK.20348 

SHELL ["C:\\buildTools\\Common7\\Tools\\VsDevCmd.bat", "&&", "powershell.exe", "-NoLogo", "-ExecutionPolicy", "Bypass"]

# Install scoop 
RUN iex (new-object net.webclient).downloadstring('https://get.scoop.sh')

# Install depedencies
RUN scoop install 7zip

# Setup llvm sources
ADD https://github.com/ghaith/llvm-package-windows/releases/download/v$LLVM_VER/LLVM-$LLVM_VER-win64.7z C:/TEMP/llvm.7z

RUN 7z x C:/TEMP/llvm.7z -ollvm 

# Install Rust
# RUN scoop install rustup
ADD https://win.rustup.rs C:/TEMP/rustup-init.exe
RUN C:/TEMP/rustup-init.exe --default-toolchain $env:RUST_VER -y
RUN rustup install nightly

RUN rustup component add llvm-tools-preview 
RUN cargo install mdbook grcov

RUN setx /M PATH $($Env:PATH + ';C:/buildTools/llvm/bin') 

ADD https://github.com/git-for-windows/git/releases/download/v2.33.0.windows.2/Git-2.33.0.2-64-bit.exe C:/TEMP/git-install.exe

RUN C:/TEMP/git-install.exe /VERYSILENT /NORESTART /DIR="C:/buildTools/Git"

#RUN cargo install cargo-watch #Activate this for local builds to enable watching
WORKDIR C:/build

#ENTRYPOINT ["cargo"]
ENTRYPOINT ["C:\\buildTools\\Common7\\Tools\\VsDevCmd.bat", "&&", "C:\\buildTools\\Git\\bin\\bash.exe", "-i", "-l"]

