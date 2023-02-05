package kz.ilotterytea.neurobaj.web.controllers.api.v1;

import com.github.twitch4j.helix.domain.User;
import io.micronaut.context.annotation.Parameter;
import io.micronaut.http.HttpResponse;
import io.micronaut.http.MediaType;
import io.micronaut.http.MutableHttpResponse;
import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Get;
import io.micronaut.http.annotation.Post;
import kz.ilotterytea.neurobaj.Neurobaj;
import kz.ilotterytea.neurobaj.utils.StringUtils;

import java.lang.management.ManagementFactory;
import java.util.*;

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

    @Post(
            value = "/join",
            produces = MediaType.APPLICATION_JSON
    )
    MutableHttpResponse<Map<String, Boolean>> joinChannels(
            @Parameter("targets") String[] targets
    ) {
        Neurobaj baj = Neurobaj.getInstance();
        ArrayList<String> s = new ArrayList<>(Arrays.asList(targets));
        ArrayList<String> ids = new ArrayList<>();
        ArrayList<String> names = new ArrayList<>();

        for (String t : s) {
            try {
                Long id = Long.parseLong(t);
                ids.add(id.toString());
            } catch (NumberFormatException e) {
                names.add(t);
            }
        }

        List<User> users = baj.getTwitchBot().getHelix().getUsers(
                baj.getProperties().getProperty("TWITCH_ACCESSTOKEN", null),
                (ids.size() > 0) ? ids : null,
                (names.size() > 0) ? names : null
        ).execute().getUsers();

        Map<String, Boolean> map = new HashMap<>();

        for (User user : users) {
            if (baj.getTwitchBot().getChat().isChannelJoined(user.getLogin())) {
                map.put(user.getLogin(), false);
            } else {
                if (baj.getTwitchBot().getChat().getChannels().size() + 1 < 100) {
                    baj.getTwitchBot().getChat().joinChannel(user.getLogin());
                    map.put(user.getLogin(), true);

                    if (!baj.getMarkov().getAllChains().containsKey(user.getId())) {
                        baj.getMarkov().getAllChains().put(user.getId(), new ArrayList<>());
                    }

                    continue;
                }
                map.put(user.getLogin(), false);
            }
        }

        return HttpResponse.ok(map);
    }

    @Post(
            value = "/part",
            produces = MediaType.APPLICATION_JSON
    )
    MutableHttpResponse<Map<String, Boolean>> partChannels(
            @Parameter("targets") String[] targets
    ) {
        Neurobaj baj = Neurobaj.getInstance();
        ArrayList<String> s = new ArrayList<>(Arrays.asList(targets));
        ArrayList<String> ids = new ArrayList<>();
        ArrayList<String> names = new ArrayList<>();

        for (String t : s) {
            try {
                Long id = Long.parseLong(t);
                ids.add(id.toString());
            } catch (NumberFormatException e) {
                names.add(t);
            }
        }

        List<User> users = baj.getTwitchBot().getHelix().getUsers(
                baj.getProperties().getProperty("TWITCH_ACCESSTOKEN", null),
                (ids.size() > 0) ? ids : null,
                (names.size() > 0) ? names : null
        ).execute().getUsers();

        Map<String, Boolean> map = new HashMap<>();

        for (User user : users) {
            if (baj.getTwitchBot().getChat().isChannelJoined(user.getLogin())) {
                baj.getTwitchBot().getChat().leaveChannel(user.getLogin());
                map.put(user.getLogin(), true);
            } else {
                map.put(user.getLogin(), false);
            }
        }

        return HttpResponse.ok(map);
    }
}
