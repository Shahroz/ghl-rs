# `chat-widget`

**8** operations / **15** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `chat-widget` cargo feature on `ghl-sdk`, then call any of the 8 generated methods on `ghl.v3().chat_widget()` (v3):

```toml
ghl-sdk = { version = "0.5", features = ["chat-widget"] }
```


## Endpoints — API v3

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `POST` | `/chat-widget/` | Create Chat Widget | `create_chat_widget()` | `v3:chat-widget.post_chat_widget` |
| `POST` | `/chat-widget/clone` | Clone Chat Widget | `clone_chat_widget()` | `v3:chat-widget.post_chat_widget_clone` |
| `GET` | `/chat-widget/data/{locationId}/{id}` | Get Chat Widget | `get_chat_widget()` | `v3:chat-widget.get_chat_widget_data_by_locationId_by_id` |
| `PATCH` | `/chat-widget/data/{locationId}/{id}` | Patch Chat Widget | `patch_chat_widget()` | `v3:chat-widget.patch_chat_widget_data_by_locationId_by_id` |
| `PUT` | `/chat-widget/data/{locationId}/{id}` | Update Chat Widget | `update_chat_widget()` | `v3:chat-widget.put_chat_widget_data_by_locationId_by_id` |
| `GET` | `/chat-widget/list` | List Chat Widgets | `list_chat_widgets()` | `v3:chat-widget.get_chat_widget_list` |
| `GET` | `/chat-widget/public/config/{id}` | Get Widget Config | `get_widget_config()` | `v3:chat-widget.get_chat_widget_public_config_by_id` |
| `DELETE` | `/chat-widget/{locationId}/{id}` | Delete Chat Widget | `delete_chat_widget()` | `v3:chat-widget.delete_chat_widget_by_locationId_by_id` |

### Endpoint details — v3

#### `POST /chat-widget/`

**Create Chat Widget**

Creates a new chat widget for the given sub-account.

Operation id: `v3:chat-widget.post_chat_widget` · `Version: v3` · Scopes: `chat-widget.write`

*Request body*: [`CreateWidgetDTO`](#createwidgetdto)

*Rust*:

```rust,ignore
let out = ghl.v3().chat_widget().create_chat_widget(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:chat-widget.post_chat_widget",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /chat-widget/clone`

**Clone Chat Widget**

Creates a copy of an existing chat widget in the same sub-account.

Operation id: `v3:chat-widget.post_chat_widget_clone` · `Version: v3` · Scopes: `chat-widget.write`

*Request body*: [`CloneChatWidgetDTO`](#clonechatwidgetdto)

*Rust*:

```rust,ignore
let out = ghl.v3().chat_widget().clone_chat_widget(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:chat-widget.post_chat_widget_clone",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /chat-widget/data/{locationId}/{id}`

**Get Chat Widget**

Returns a single chat widget by ID.

Operation id: `v3:chat-widget.get_chat_widget_data_by_locationId_by_id` · `Version: v3` · Scopes: `chat-widget.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | The chat widget ID |
| `locationId` | string | **yes** | The location ID |

*Rust*:

```rust,ignore
let out = ghl.v3().chat_widget().get_chat_widget(&id, &locationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:chat-widget.get_chat_widget_data_by_locationId_by_id",
    "path_params": {
      "id": "<id>",
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PATCH /chat-widget/data/{locationId}/{id}`

**Patch Chat Widget**

Partial update of a chat widget resource.

Operation id: `v3:chat-widget.patch_chat_widget_data_by_locationId_by_id` · `Version: v3` · Scopes: `chat-widget.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | The chat widget ID |
| `locationId` | string | **yes** | The location ID |

*Request body*: [`UpdateWidgetDTO`](#updatewidgetdto)

*Rust*:

```rust,ignore
let out = ghl.v3().chat_widget().patch_chat_widget(&id, &locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:chat-widget.patch_chat_widget_data_by_locationId_by_id",
    "path_params": {
      "id": "<id>",
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /chat-widget/data/{locationId}/{id}`

**Update Chat Widget**

Full update of a chat widget resource.

Operation id: `v3:chat-widget.put_chat_widget_data_by_locationId_by_id` · `Version: v3` · Scopes: `chat-widget.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | The chat widget ID |
| `locationId` | string | **yes** | The location ID |

*Request body*: [`UpdateWidgetDTO`](#updatewidgetdto)

*Rust*:

```rust,ignore
let out = ghl.v3().chat_widget().update_chat_widget(&id, &locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:chat-widget.put_chat_widget_data_by_locationId_by_id",
    "path_params": {
      "id": "<id>",
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /chat-widget/list`

**List Chat Widgets**

Returns chat widgets for the sub-account with pagination and optional filters.

Operation id: `v3:chat-widget.get_chat_widget_list` · `Version: v3` · Scopes: `chat-widget.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | The location ID |
| `offset` | string | **yes** | Offset |
| `limit` | string | **yes** | Limit |
| `chatType` | enum: `liveChat`, `waChat`, `emailChat`, `allInOneChat`, `voiceAiChat`, `facebookChat`, `instagramChat`, `webChat` | no | The type of chat widget. Supports normal ChatType values, plus the virtual umbrella "webChat" (maps to facebookChat/emailChat/instagramChat/waChat). |
| `excludeChatType` | enum: `liveChat`, `waChat`, `emailChat`, `allInOneChat`, `voiceAiChat`, `facebookChat`, `instagramChat` | no | The type of chat widget |
| `voiceAiAgentId` | string | no | The voice AI agent ID |
| `allInOneChatTypes` | enum: `liveChat`, `waChat`, `emailChat`, `allInOneChat`, `voiceAiChat`, `facebookChat`, `instagramChat`, `webChat` | no | All-in-one chat type to filter by. Only applies when chatType is "allInOneChat". Supports normal ChatType values plus the virtual umbrella "webChat" (maps to fa… |

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::chat_widget::ListChatWidgetsParams;

let params = ListChatWidgetsParams::new("locationId", "offset", "limit");
let out = ghl.v3().chat_widget().list_chat_widgets(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:chat-widget.get_chat_widget_list",
    "query": {
      "locationId": "<locationId>",
      "offset": "<offset>",
      "limit": "<limit>"
    }
  }
}
```

</details>

#### `GET /chat-widget/public/config/{id}`

**Get Widget Config**

Returns widget configuration by ID.

Operation id: `v3:chat-widget.get_chat_widget_public_config_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | The chat widget ID |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `version` | string | no | — |

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::chat_widget::GetWidgetConfigParams;

let params = GetWidgetConfigParams::new();
let out = ghl.v3().chat_widget().get_widget_config(&id, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:chat-widget.get_chat_widget_public_config_by_id",
    "path_params": {
      "id": "<id>"
    }
  }
}
```

</details>

#### `DELETE /chat-widget/{locationId}/{id}`

**Delete Chat Widget**

Soft-deletes a chat widget. If it was the default, another widget may be promoted.

Operation id: `v3:chat-widget.delete_chat_widget_by_locationId_by_id` · `Version: v3` · Scopes: `chat-widget.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | The chat widget ID |
| `locationId` | string | **yes** | The location ID |

*Rust*:

```rust,ignore
let out = ghl.v3().chat_widget().delete_chat_widget(&id, &locationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:chat-widget.delete_chat_widget_by_locationId_by_id",
    "path_params": {
      "id": "<id>",
      "locationId": "<locationId>"
    }
  }
}
```

</details>

## Data models — API v3

In Rust: `ghl_models::v3::chat_widget::*` (enable the `chat-widget` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/chat_widget/).

### `A2PComplianceDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `enableA2PCompliance` | bool | no | Enable A2P Compliance |
| `a2pOptInForm1` | String | no | A2P Opt In Form 1 |
| `a2pOptInForm1ShowCheckbox` | bool | no | Show checkbox for A2P Opt In Form 1 |
| `a2pOptInForm1PreChecked` | bool | no | Pre-checked state for A2P Opt In Form 1 checkbox |
| `isA2POptInForm2` | bool | no | Is A2P Opt In Form 2 |
| `a2pOptInForm2` | String | no | A2P Opt In Form 2 |
| `a2pOptInForm2ShowCheckbox` | bool | no | Show checkbox for A2P Opt In Form 2 |
| `a2pOptInForm2PreChecked` | bool | no | Pre-checked state for A2P Opt In Form 2 checkbox |
| `privacyPolicyLink` | String | no | Privacy Policy Link |
| `termsOfServiceLink` | String | no | Terms of Service |
| `isA2POptInForm1` | bool | no | Is A2P Opt In Form 1 enabled |
| `messageType` | String | no | Message Type |

### `AcknowledgementDetailsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `icon` | String | no | Icon |
| `placeholderColor` | String | no | Placeholder color |
| `liveChatIcon` | String | no | Icon for live chat |
| `liveChatPlaceholderColor` | String | no | Placeholder color for live chat |

### `AdvanceSettingsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `brandingTitle` | String | no | Branding Title |
| `redirect` | [`RedirectDTO`](#redirectdto) | no | Redirect Object |
| `enableContactForm` | bool | no | Boolean for showing contact form at start |
| `defaultConsentCheck` | bool | no | By default consent check for contact form |
| `businessOfficeHours` | [`BusinessOfficeHoursDTO`](#businessofficehoursdto) | no | Business Office Hours |
| `contactFormOptions` | Vec<String> | no | Contact form field configuration |
| `allInOneChatTypes` | Vec<String (enum)> | no | Chat types included in the all-in-one widget |
| `allInOneInitialMsg` | String | no | All In One Initial Msg |
| `contactFormIntroMessage` | String | no | Contact Form Intro Message |
| `contactFormSystemMessage` | String | no | Contact Form System Message |
| `prefilledMessageText` | String | no | Prefilled Message Text |
| `voiceAiAgent` | JSON | no | Voice AI Agent |
| `fbPage` | [`FBPageDTO`](#fbpagedto) | no | Facebook Page |
| `instagramPage` | [`InstagramPageDTO`](#instagrampagedto) | no | Instagram Page |
| `playNotificationSound` | bool | no | Play Notification Sound |
| `voiceAiSendActionText` | String | no | Voice Ai Send Action Text |
| `aTwoPCompliance` | [`A2PComplianceDTO`](#a2pcompliancedto) | no | A2P Compliance |

### `BusinessOfficeHoursDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `enableBusinessHours` | bool | no | Enable Business Hours |
| `openHours` | Vec<String> | no | Open hours schedule |
| `timezone` | String | no | Time Zone |
| `outsideOfficeHoursWelcomeMsg` | String | no | Time Zone |

### `CloneChatWidgetDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | locationId |
| `chatWidgetId` | String | **yes** | chat widget ID |
| `name` | String | no | Name for the cloned widget |

### `CreateWidgetDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `version` | f64 | **yes** | Version |
| `chatType` | String — `liveChat`, `emailChat` | **yes** | Chat type |
| `name` | String | **yes** | Name |
| `locationId` | String | **yes** | Location ID |
| `deleted` | bool | no | Deleted |
| `default` | bool | no | Default |
| `settings` | [`WidgetSettingsDTO`](#widgetsettingsdto) | no | Settings |

### `FBPageDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `facebookPageId` | String | no | Facebook Page ID |
| `facebookPageName` | String | no | Facebook Page Name |

### `InstagramPageDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `facebookPageId` | String | no | Facebook Page ID |
| `facebookPageName` | String | no | Facebook Page Name |
| `instagramPageId` | String | no | Instagram Page ID |
| `instagramUsername` | String | no | Instagram UserName |

### `RedirectDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `redirectAction` | bool | no | Redirect Action |
| `redirectWebsite` | String | no | Redirect Website |
| `redirectText` | String | no | Redirect Text |

### `UpdateWidgetDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `version` | f64 | no | Version |
| `chatType` | String — `liveChat`, `emailChat` | no | Chat type |
| `name` | String | no | Name |
| `default` | bool | no | Default |
| `settings` | [`WidgetSettingsDTO`](#widgetsettingsdto) | no | Settings |

### `WidgetSettingsCustomizationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `position` | String | no | Position |
| `sizes` | [`WidgetSettingsCustomizationSizeDTO`](#widgetsettingscustomizationsizedto) | no | Typography Color Options |

### `WidgetSettingsCustomizationSizeDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `width` | f64 | no | Width |
| `height` | f64 | no | Height |

### `WidgetSettingsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `acknowledgementDetails` | [`AcknowledgementDetailsDTO`](#acknowledgementdetailsdto) | no | Acknowledgement Details |
| `agencyName` | String | no | Name of the agency |
| `agencyWebsite` | String | no | Website URL of the agency |
| `allowAvatarImage` | bool | no | Allow avatar image |
| `autoCountryCode` | bool | no | Boolean indicating whether to automatically detect country code |
| `countryCode` | String | no | Country code |
| `chatType` | String | **yes** | Chat Type |
| `promptType` | String | no | Prompt Type |
| `chatIcon` | String | **yes** | Message Chat Circle |
| `enableRevisitMessage` | bool | no | Boolean indicating whether to enable a revisit message |
| `heading` | String | no | Heading text |
| `legalMsg` | String | no | Legal message |
| `liveChatAckMsg` | String | no | Message acknowledging a live chat |
| `liveChatEndedMsg` | String | no | Message indicating the end of a live chat |
| `liveChatFeedbackMsg` | String | no | Message asking for feedback after a live chat |
| `liveChatFeedbackNote` | String | no | Note regarding live chat feedback |
| `liveChatIntroMsg` | String | no | Introduction message for a live chat |
| `liveChatUserInactiveMsg` | String | no | Message for inactive users during a live chat |
| `liveChatUserInactiveTime` | String | no | Time for considering a user inactive during a live chat |
| `liveChatVisitorInactiveMsg` | String | no | Message for inactive visitors during a live chat |
| `liveChatVisitorInactiveTime` | String | no | Time for considering a visitor inactive during a live chat |
| `locale` | String | no | Locale setting |
| `promptAvatar` | String | no | Avatar for prompts |
| `promptAvatarAltText` | String | no | Prompt Avatar Alt Text |
| `isPromptAvatarImageOptimize` | bool | no | Avatar Image Optimization |
| `promptMsg` | String | no | Prompt message |
| `revisitPromptMsg` | String | no | Message for revisiting prompts |
| `sendActionText` | String | no | Text for send action |
| `showAgencyBranding` | bool | no | Boolean indicating whether to show agency branding |
| `showConsentCheckbox` | bool | no | Boolean indicating whether to show a consent checkbox |
| `showLiveChatWelcomeMsg` | bool | no | Boolean indicating whether to show a welcome message for live chat |
| `showPrompt` | bool | no | Boolean indicating whether to show prompts |
| `subHeading` | String | no | Subheading text |
| `successMsg` | String | no | Success message |
| `supportContact` | String | no | Contact information for support |
| `thankYouMsg` | String | no | Thank you message |
| `theme` | [`WidgetSettingsThemeDTO`](#widgetsettingsthemedto) | no | Theme |
| `useEmailField` | bool | no | Boolean indicating whether to use an email field |
| `waNumber` | String | no | WhatsApp number |
| `widgetPrimaryColor` | String | no | Primary color for the widget |
| `representativeAssignedMessage` | String | no | Representative Assigned Message |
| `dimensions` | [`WidgetSettingsCustomizationDTO`](#widgetsettingscustomizationdto) | no | Customizations |
| `advanceSettings` | [`AdvanceSettingsDTO`](#advancesettingsdto) | no | Advance Settings |
| `locationCountryCode` | String | no | Location Country Code |
| `widgetId` | String | no | Widget Id |
| `widgetPlacement` | String | no | Widget Placement |

### `WidgetSettingsThemeCustomColorDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `chatBubbleColor` | String | no | Chat Bubble Color |
| `backgroundColor` | String | no | Background Color |
| `headerColor` | String | no | Header Color |
| `buttonColor` | String | no | Button Color |
| `avatarBackgroundColor` | String | no | Avatar Background Color |
| `avatarBorderColor` | String | no | Avatar Border Color |
| `senderMessageColor` | String | no | Sender Message Color |
| `receivedMessageColor` | String | no | Received Message Color |

### `WidgetSettingsThemeDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Theme Name |
| `colors` | [`WidgetSettingsThemeCustomColorDTO`](#widgetsettingsthemecustomcolordto) | no | Custom Color Options |

