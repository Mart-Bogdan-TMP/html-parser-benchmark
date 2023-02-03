package com.innahema.java_parser_benchmark;

import org.openjdk.jmh.annotations.*;
import org.openjdk.jmh.infra.Blackhole;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.TimeUnit;

public class StreamBench {

    static final int LIST_SIZE = 80000;
    static List<Integer> LIST;

    static {
        LIST = new ArrayList<>(LIST_SIZE);
        for (int i = 0; i < LIST_SIZE; i++) {
            LIST.add(i);
        }
    }
/*
    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.MILLISECONDS)
    public void loop(Blackhole bh){
        for (Integer x : LIST) {
            bh.consume(x);
        }
    }

    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.MILLISECONDS)
    public void forEach_ref(Blackhole bh){
        LIST.forEach(bh::consume);
    }


    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.MILLISECONDS)
    public void forEach_lambda(Blackhole bh){
        //noinspection Convert2MethodRef
        LIST.forEach(obj -> bh.consume(obj));
    }
*/
    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.MICROSECONDS)
    @CompilerControl(CompilerControl.Mode.INLINE)
    public int sumStream_INLINE(){
        int sum = LIST.stream()
                .mapToInt(x -> x)
                .filter(x -> x % 2 == 0)
                .map(x -> x + 3)
                .sum();
        return sum;
    }
    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.MICROSECONDS)
    @CompilerControl(CompilerControl.Mode.EXCLUDE)
    public int sumStream_EXCLUDE(){
        int sum = LIST.stream()
                .mapToInt(x -> x)
                .filter(x -> x % 2 == 0)
                .map(x -> x + 3)
                .sum();
        return sum;
    }
    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.MICROSECONDS)
    public int sumStream(){
        int sum = LIST.stream()
                .mapToInt(x -> x)
                .filter(x -> x % 2 == 0)
                .map(x -> x + 3)
                .sum();
        return sum;
    }
    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.MICROSECONDS)
    @CompilerControl(CompilerControl.Mode.COMPILE_ONLY)
    public int sumStream_COMPILE_ONLY(){
        int sum = LIST.stream()
                .mapToInt(x -> x)
                .filter(x -> x % 2 == 0)
                .map(x -> x + 3)
                .sum();
        return sum;
    }
    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.MICROSECONDS)
    @CompilerControl(CompilerControl.Mode.DONT_INLINE)
    public int sumStream_DONT_INLINE(){
        int sum = LIST.stream()
                .mapToInt(x -> x)
                .filter(x -> x % 2 == 0)
                .map(x -> x + 3)
                .sum();
        return sum;
    }
//    @Benchmark
//    @BenchmarkMode(Mode.AverageTime)
//    @OutputTimeUnit(TimeUnit.MICROSECONDS)
    public int sumLoop(){
        int sum = 0;
        for (Integer x : LIST) {
            int i = x;
            if (i % 2 == 0) {
                int i1 = i + 3;
                sum += i1;
            }
        }
        return sum;
    }

//    @Benchmark
//    @BenchmarkMode(Mode.AverageTime)
//    @OutputTimeUnit(TimeUnit.MICROSECONDS)
    public int sumLoopRef(){
        int sum = 0;
        for (Integer x : LIST) {
            if (x % 2 == 0) {
                sum += x + 3;
            }
        }
        return sum;
    }
}
