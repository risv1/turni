from airflow import DAG
from datetime import datetime, timedelta
from airflow.operators.bash import BashOperator

default_args = {
    "owner": "airflow",
    "retries": 1,
    "retry_delay": timedelta(minutes=2)
}

with DAG(
    dag_id="echo_dag",
    default_args=default_args,
    description="A DAG that echos a message",
    start_date=datetime(2024, 11, 14, 2),
    schedule_interval="@daily"
) as dag:
    echo_hello = BashOperator(
        task_id="echo_hello",
        bash_command="echo 'Hello, World!'"
    )

    echo_hello