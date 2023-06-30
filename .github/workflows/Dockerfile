# docker build -t ghcr.io/username/repo-name .
# docker login ghcr.io -u username -p yourPAT
# docker push ghcr.io/username/repo-name

FROM quay.io/pypa/manylinux2014_aarch64

# Install necessary development headers
RUN yum install -y glibc-devel
