package com.innahema.java_parser_benchmark;

import org.apache.commons.io.FileUtils;
import org.jsoup.Jsoup;
import org.jsoup.nodes.Document;
import org.openjdk.jmh.annotations.Benchmark;
import org.openjdk.jmh.annotations.BenchmarkMode;
import org.openjdk.jmh.annotations.Mode;
import org.openjdk.jmh.annotations.OutputTimeUnit;

import java.io.File;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.concurrent.TimeUnit;

public class JsoupBenchmark {

    static String html;
    static String htmlBig;
    static byte[] htmlBytes;
    static {
        try {
            File sampleFileName = new File("../sample.html");

            html = FileUtils.readFileToString(sampleFileName, StandardCharsets.UTF_8);
            htmlBytes = FileUtils.readFileToByteArray(sampleFileName);
            htmlBig = FileUtils.readFileToString(new File("../big.html"), StandardCharsets.UTF_8);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

//    @Benchmark
//    @BenchmarkMode(Mode.AverageTime)
//    @OutputTimeUnit(TimeUnit.MILLISECONDS)
    public Document pareseDom() {
        return Jsoup.parse(html);
    }

//    @Benchmark
//    @BenchmarkMode(Mode.AverageTime)
//    @OutputTimeUnit(TimeUnit.MILLISECONDS)
    public Document pareseDomBig() {
        return Jsoup.parse(htmlBig);
    }

//    @Benchmark
//    @BenchmarkMode(Mode.AverageTime)
//    @OutputTimeUnit(TimeUnit.MILLISECONDS)
//    public Document pareseDomFromBytes() {
//        String htmlStr = new String(htmlBytes, StandardCharsets.UTF_8);
//        return Jsoup.parse(htmlStr);
//    }


}
