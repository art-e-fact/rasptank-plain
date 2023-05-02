FROM balenalib/rpi-raspbian:bullseye-20221229



RUN pip install --upgrade setuptools
RUN pip install artefacts-client --extra-index-url https://d5cw4z7oemmfd.cloudfront.net/pep503/ -U

CMD artefacts run $ARTEFACTS_JOB_NAME
