package com.innahema.java_parser_benchmark;

import org.apache.commons.io.FileUtils;
import org.jsoup.Jsoup;
import org.jsoup.nodes.Document;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.io.File;
import java.nio.charset.StandardCharsets;

import static org.junit.jupiter.api.Assertions.*;

class JsoupBenchmarkTest {

    @Test
    void testFileLoads() throws Exception {
        String html = FileUtils.readFileToString(new File("../sample.html"), StandardCharsets.UTF_8);
        Assertions.assertNotNull(html);
    }

    @Test
    void testParses() throws Exception {
        String html = FileUtils.readFileToString(new File("../sample.html"), StandardCharsets.UTF_8);
        Document document = Jsoup.parse(html);
        Assertions.assertEquals("Head1", document.head().attr("id"));
    }

}