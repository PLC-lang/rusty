FUNCTION main
    VAR
        impl : FbImpl;
        consumer : FbConsumer;
    END_VAR

    consumer.setRef(impl);
    printf('%d$N', consumer.invoke());
END_FUNCTION
