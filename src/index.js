export default {
  async fetch(request, env) {
    // Only allow POST requests
    if (request.method !== "POST") {
      return new Response("Please use POST method", { status: 405 });
    }

    try {
      // Parse the JSON body from the request
      const { dreamPrompt } = await request.json();

      if (!dreamPrompt) {
        return new Response("Missing dreamPrompt in request body", { status: 400 });
      }

      // Prepare the chat messages for the AI
      const chat = {
        messages: [
          { 
            role: 'system', 
            content: `You are a knowledgeable and approachable sleep and dream expert. 
                      Analyze dream descriptions and provide insights, but maintain a tone 
                      that suggests you're offering possibilities rather than definitive answers. 
                      Suggest a few potential reasons for why the dream might have occurred.
                      
                      The response should read just like another human directly responding naturally.
                      
                      This is a one-off response and must not prompt the user to continue the conversation.`
          },
          { role: 'user', content: `Analyze this dream: ${dreamPrompt}` }
        ]
      };

      // Run the AI model
      const response = await env.AI.run('@cf/mistral/mistral-7b-instruct-v0.1', chat);

      // Return the AI's response
      return new Response(JSON.stringify({ analysis: response }), {
        headers: { 'Content-Type': 'application/json' }
      });

    } catch (error) {
      return new Response(`Error: ${error.message}`, { status: 500 });
    }
  }
};