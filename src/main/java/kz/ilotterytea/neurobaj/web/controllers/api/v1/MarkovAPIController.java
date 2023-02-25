package kz.ilotterytea.neurobaj.web.controllers.api.v1;

import io.micronaut.http.*;
import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Get;
import kz.ilotterytea.neurobaj.Neurobaj;
import kz.ilotterytea.neurobaj.neural.markov.ChatChain;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.Map;

/**
 * @author ilotterytea
 * @since 1.0
 */
@Controller("/api/v1/markov/")
public class MarkovAPIController {

    @Get(
            value = "/ask",
            produces = MediaType.TEXT_PLAIN
    )
    MutableHttpResponse<String> getResponse(
            String message
            ) {
        if (Neurobaj.getInstance() == null) {
            return HttpResponse.status(HttpStatus.INTERNAL_SERVER_ERROR).body("ERROR: Neurobaj not initialized!");
        }

        String response = Neurobaj.getInstance().getMarkov().generateText(message);

        return HttpResponse.ok(response);
    }

    @Get(
            value = "/status",
            produces = MediaType.APPLICATION_JSON
    )
    MutableHttpResponse<Map<String, Object>> getStatus() {
        Map<String, Object> map = new HashMap<>();
        Map<String, ArrayList<ChatChain>> chains = Neurobaj.getInstance().getMarkov().getAllChains();
        Map<String, Integer> chainCountPerChannel = new HashMap<>();

        int totalChains = 0;

        for (int i = 0; i < chains.values().size(); i++) {
            totalChains += new ArrayList<>(chains.values()).get(i).size();

            chainCountPerChannel.put(new ArrayList<>(chains.keySet()).get(i), new ArrayList<>(chains.values()).get(i).size());
        }

        map.put("total_chains", totalChains);
        map.put("total_chains_per_channel", chainCountPerChannel);

        return HttpResponse.ok(map);
    }
}
