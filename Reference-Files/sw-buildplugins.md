How do I build Plugins?

Last updated on March 20, 2024

Print
Creating Plugins
Roll up your sleeves and start making some Plugins of your own—for yourself or everyone on Sudowrite! If you know some basics when it comes to writing prompts for AI, building your first Plugin will be easy.

But before you get started, make sure to set your profile name to what you want publicly displayed. Whatever you set here is what will show up as the Creator Name on Plugins you Publish to the Directory.

To get started, click on the Create Plugin button in the top right corner of the plugins list page to get to the plugin creation flow.

 
The Basic Editor

Notion Image
The Basic Editor is the quickest way to build a Plugin, and you can do so with something as simple as a single prompt.

Here’s what each of the fields pictured above does:

Name - Give your Plugin a unique name. The name should be succinct, and clearly communicate what the Plugin does since this will be the name seen in the dropdown.
Description - Describe what the Plugin does and include specifics of how to use it. Depending on the Plugin, it may make sense to include an example input and output response.
Visibility - You can choose to Publish your Plugin or keep it Unlisted. Unlisted plugins do not show up in the Directory or search results, but you can still use them—and anyone with the direct URL would also be able to see/install the Plugin.
Category - Choose a single option from the options available to categorize your Plugin according to what it is for—anything from Analysis & Feedback to World-Building. This will determine where the Plugin appears on the Explore page.
The Categories currently available are:
Your Instructions - This is where you tell the AI exactly what to do. When a user highlights text and then uses your Plugin, the AI will read the text first, and then be prompted with these instructions. Be precise with your instructions, as if it were written for an assistant who cannot ask you follow-up questions while they are doing the task.
Enable Story Bible Data - If your Plugin could benefit from understanding the user’s Story Bible, enable this and click on the relevant parts of the Story Bible that you’d like to share with the AI. (Maybe the plugin needs to know the genre or style to work optimally—it’s up to you!)
 
🧠
Good to Know
Plugins use credits, just like other Sudowrite features, when used. Basic Plugins are generally pretty cost effective, but might get expensive if you decide to pass in the complete contents of someone’s Story Bible.
Advanced Plugins get way more complex, letting you insert variables where and how you’d like them, toggle between AI models, and even string prompts together in sequence—which means the credit use could vary dramatically.
 
Advanced Editor

If you need more control over the prompts, the Advanced Editor gives you precise control over the prompt formatting and various Large Language Model options. You can also create multi-stage prompts (currently up to 2)!

Notion Image
The Advanced Editor begins to look different from the Basic Editor once you scroll past the visibility and category options.
Here’s what each of the areas in the screenshot above controls:

Preceding Text - Preceding Text is a variable representing the words before the cursor. In Basic Plugins this works as a fallback, reading the words before the cursor if the Plugin user had not highlighted anything. If you plan on using {{ preceding_text }} then you may want to specify the minimum and/or maximum number of words needed before the cursor for the Plugin to work here.
Highlighted Text - Highlighted Text is a variable representing the words the user has highlighted—this will be replaced by the exact words the user has highlighted when the Plugin is run, passing that context into the prompt. If you plan on using {{ highlighted_text }} you can specify the minimum words required and maximum allowed for a highlight here.
User Text Input - If you toggle on Allow Users to Give Instructions, you also enable the {{ user_text_input }} variable. With this feature enabled, your Plugin users will get a popup window before the Plugin is actually run, prompting them for input according to instructions you’ve defined. Inserting this variable in your Plugin instructions is a way to directly pass in end user input.
Notion Image
 
💱
Available Variables - In addition to those discussed above (which have their own configurator inside of the Advanced Plugin builder) there are a bunch of additional variables available to enrich your custom Plugins.
{{ previous_document_text }} - The text from all previous documents in the chain of previous documents. This takes advantage of a user’s chapter continuity.
{{ braindump }} - The braindump from the story bible.
{{ genre }} - The genre from the user’s story bible.
{{ style }} - The style from the user’s story bible.
{{ synopsis }} - The synopsis from the user’s story bible.
{{ characters }} - The characters from the user’s story bible, optimized by the Saliency engine to only include textually relevant character data.
{{ characters_raw }} - All of the character data from the story bible, raw, unoptimized.
{{ outline }} - The outline from the story bible.
{{ scene_summary }} - The section of the outline linked with the current document a plugin is being used within, if any.
{{ is_story_bible_active }} - Information as to whether or not story bible is toggled on.
{{ chapter_scenes }} - The Scenes from the story bible.
{{ chapter_scenes_extra_instructions }} - The Extra Instructions that a user has opted to include in their Draft box.
{{ worldbuilding }} - The worldbuilding entries from the story bible, optimized by the Saliency engine to only include what is textually relevant.
{{ worldbuilding_raw }} - The worldbuilding entries from the story bible, raw, unoptimized.

 
Prompts - By default an Advanced Plugin has a simple Prompt area—Prompt 1. Inside of that box you can specify the exact instructions and LLM you’d like.
System - This specifies how the AI should behave. For instance, a system message could be: "You are a helpful assistant." This instructs the model to behave like a helpful assistant during the chat.
User - This is the substance of your plugin. Here, you will combine the user’s {{ highlighted_text }}  and possibly the Story Bible data into a prompt for the language model to produce the desired functionality for the user. A few guidelines:
Give a strong cue as to what part of the prompt is the user’s highlighted text, which will let the AI know what part is coming from the user. Something like: 

Here is a passage of text:

{{ highlighted_text }}

Based on the passage of text provided, [...rest of instructions...]
Be as specific as you can with your instructions. A good approach is to write this as if you’re writing an email to an assistant for them to do the task correctly on the first try without requiring clarifying questions.
If prompt rewrites the user’s highlighted text, specify at the end how long the rewrite should be. If it should be rewriting into approximately the same number of words, say so, like ONLY REWRITE THE PASSAGE, DO NOT MAKE IT LONGER
The genre, style, synopsis, characters, and outline are available as variables as well, allowing you to inject Story Bible data to influence the prompt.
AI Options - These options changes the behavior of the language model selected. 
Engine - You can select from several different AI models. We believe gpt-4o-mini is sufficient for most tasks—but much more capable (albeit slower and more expensive) models exist, such as gpt-4.1 and even gemini-2.5-pro which is capable of reading entire manuscripts.
Temperature - Think of this as the "creativity" dial. A higher temperature (e.g., 1.0) makes the model's responses more random and creative, while a lower temperature (e.g., 0.1) makes them more focused and deterministic. Ranges from 0 to 1.0. We recommend 0.85 for most use cases.
Frequency Penalty - Controls how much the model avoids or prefers frequently used words or phrases. A negative value makes it more likely to use frequent words, while a positive value makes it avoid them. It's like asking a writer to avoid clichés: a positive value tells them to be more original and not use common phrases, while a negative value lets them use those familiar phrases freely.
Presence Penalty - This adjusts the model's inclination to include new ideas or topics. A positive value encourages it to introduce new concepts, while a negative value makes it stick more closely to the topics already mentioned. Imagine instructing a teacher: a positive value tells them to bring up new topics often, while a negative one tells them to stay on the current topic.
Stop - This parameter lets you specify words that, when generated by the model, will make it stop generating any further. For instance, if you set "stop" to be ".", the model would stop generating text as soon as it produces a period. It's like giving a speaker a cue: "When you mention this word or symbol, wrap up your speech.”
Number of Generations - The number of cards your plugin should generate. Each card is an independent “run” of your plugin, and do not affect each other. Specifying more than one is useful if you want to give the user a diverse set of options to choose from. (Note: multi-stage prompts require this to be 1)
Max Tokens - This determines the maximum number of tokens the model will produce in its output (where a token is roughly 0.75 words). For example, if you set "max tokens" to 50, the model will not generate a response longer than 50 tokens.
Multi-Stage Prompts - To create multi-stage prompts—which are essentially two of these in sequence—you can click on the “+ Prompt” button at the bottom, which will create a new prompt. The second stage is exactly the same as the first, except now you have access to a new variable {{ prompt_1_result }} which will be the output from the first prompt. This is great if you’d like to run an analysis, and the use that analysis to run an edit or text transformation in the following prompt.
Note that while you will see the prompt 1 result in testing, only the end result of the second prompt will be output to a card in the user’s History when a live Plugin is used. This means users will not see the intermediate prompt results. (In the example above, the end user would not see the analysis, only the transformed text.)
⚙
Heads Up
Once you start editing your Plugin in the Advanced Editor, you may not be able to go back to the Basic Editor—especially if you use options that the Basic Editor does not support.
 
Testing Plugins
At the bottom of the Plugin creation page, you will see a testing area:

Notion Image
This lets you easily and quickly test your Plugin without publishing it. It’s essential that you test your plugin to make sure it works with a diverse set of inputs—and that you’re getting the results you want out of it. We suggest that you have a bank of inputs along with an idea what your expected output is. This way, as you iterate on the design of your plugin, you can make sure that the functionality matches your expectations.

Toggling Additional Variables exposes fields to populate Preceding Text as well as any Story Bible test data (in case your Plugin makes use of those). Note that we’ve pre-populated some Story Bible context for quick testing, but you can replace that with more tailored inputs for better results. (If you notice your Plugin is talking a lot about Bigfoot in testing, it’s possible you’re passing in this context without realizing it!)

Profile Name
You can edit what name shows up for your Plugins by clicking on the small “Edit” link in the Settings (⚙️) menu in top right of your Sudowrite interface:

Notion Image
By default, if you’ve logged in via Google, that will be set to your Google display name. 

Example - Summarize Plot Points
 
How does the Saliency Engine work with Plugins?
Some plugins are designed to use your Story Bible data. In those cases, Saliency Engine may or may not be used, depending entirely on how the creator built that plugin.

If the creator chose to use the {{ characters }} or {{ worldbuilding }} variables, the Saliency Engine will kick in when necessary.
If the creator chose to use the {{ characters_raw }} or {{ worldbuilding_raw }} variables, the full context of those fields will be passed along into the AI in their entirety. The Saliency Engine will make no attempt to parse the data for relevant context.
Note: In most cases, the raw version will give the AI way more context. This can either degrade or improve the results, depending on both the plugin configuration and the context passed through as a result… but it will almost always make the Plugin use more credits.
Hidden characters or traits will never be visible to the AI, even if a Plugin is using the a raw variable.
🚧
Be Advised: Credit cost is always calculated based on input, output, and the model selected. If a user with hundreds of Characters or Worldbuilding Elements uses a Plugin that includes the raw version of a variable, a great deal of context will be passed through to the AI. This could consume a ton of credits!
All Plugins that predate the Saliency Engine use the standard {{ characters }} variable, and so will use Saliency Engine by default.