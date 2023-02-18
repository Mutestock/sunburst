FROM python:3.10

COPY proto /proto
COPY backend/py_rest /backend/py_rest
COPY commons/py /commons/py
COPY config.toml /config.toml
WORKDIR /backend/py_rest
RUN pip install poetry
ENV POETRY_HOME=/opt/poetry
ENV POETRY_VIRTUALENVS_IN_PROJECT=true
ENV PATH="$POETRY_HOME/bin:$PATH"
ENV PATH /root/.local/bin:$PATH
RUN poetry export > requirements.txt
RUN pip install -r requirements.txt

EXPOSE 22551

CMD [ "python", "py_rest/main.py" ]  