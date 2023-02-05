package kz.ilotterytea.neurobaj.web.controllers.api.v1;

import io.micronaut.http.*;
import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Get;
import kz.ilotterytea.neurobaj.Neurobaj;

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
}
