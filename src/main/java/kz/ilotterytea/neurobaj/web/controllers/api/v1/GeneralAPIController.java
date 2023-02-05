package kz.ilotterytea.neurobaj.web.controllers.api.v1;

import io.micronaut.http.HttpResponse;
import io.micronaut.http.MediaType;
import io.micronaut.http.MutableHttpResponse;
import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Get;
import kz.ilotterytea.neurobaj.Neurobaj;
import kz.ilotterytea.neurobaj.utils.StringUtils;

import java.lang.management.ManagementFactory;
import java.util.HashMap;
import java.util.Map;

/**
 * @author ilotterytea
 * @since 1.0
 */
@Controller("/api/v1/")
public class GeneralAPIController {

    @Get(
            value = "/status",
            produces = MediaType.APPLICATION_JSON
    )
    MutableHttpResponse<Map<String, Object>> getStatus() {
        Map<String, Object> map = new HashMap<>();
        Runtime rt = Runtime.getRuntime();
        long uptime = ManagementFactory.getRuntimeMXBean().getUptime();

        map.put("uptime_raw", uptime);
        map.put("uptime_formatted", StringUtils.formatTimestamp(uptime / 1000));

        map.put("used_mem_mb", Math.round(((rt.totalMemory() - rt.freeMemory()) / 1024.0) / 1024.0));
        map.put("total_mem_mb", Math.round(((rt.totalMemory()) / 1024.0) / 1024.0));
        map.put("latency", (Neurobaj.getInstance().getTwitchBot() != null && Neurobaj.getInstance().getTwitchBot().getChat() != null) ? Neurobaj.getInstance().getTwitchBot().getChat().getLatency() : -1);
        map.put("java", System.getProperty("java.version"));

        return HttpResponse.ok(map);
    }
}
