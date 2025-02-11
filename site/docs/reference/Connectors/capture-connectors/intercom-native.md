
# Intercom

This connector captures data from Intercom into Flow collections.

It is available for use in the Flow web application. For local development or open-source workflows, [`ghcr.io/estuary/source-intercom:dev`](https://ghcr.io/estuary/source-intercom-native:dev) provides the latest version of the connector as a Docker image. You can also follow the link in your browser to see past image versions.

## Supported data resources

The following data resources are supported through the Intercom API:

* [Admins](https://developers.intercom.com/intercom-api-reference/reference/list-admins)
* [Companies](https://developers.intercom.com/intercom-api-reference/reference/list-companies)
* [Company attributes](https://developers.intercom.com/intercom-api-reference/reference/list-data-attributes)
* [Company segments](https://developers.intercom.com/intercom-api-reference/reference/list-attached-segments-1)
* [Contacts](https://developers.intercom.com/intercom-api-reference/reference/list-contacts)
* [Contact attributes](https://developers.intercom.com/intercom-api-reference/reference/list-data-attributes)
* [Conversations](https://developers.intercom.com/intercom-api-reference/reference/list-conversations)
* [Conversation parts](https://developers.intercom.com/intercom-api-reference/reference/retrieve-a-conversation)
* [Segments](https://developers.intercom.com/intercom-api-reference/reference/list-segments)
* [Tags](https://developers.intercom.com/intercom-api-reference/reference/list-tags-for-an-app)
* [Teams](https://developers.intercom.com/intercom-api-reference/reference/list-teams)

By default, each resource is mapped to a Flow collection through a separate binding.

## Prerequisites

There are two ways to authenticate with Intercom: using OAuth2, or with an [access token](https://developers.intercom.com/building-apps/docs/authentication-types#section-how-to-get-your-access-token).

OAuth is recommended for simplicity in the Flow web app.

## Configuration

You configure connectors either in the Flow web app, or by directly editing the catalog specification file.
See [connectors](../../../concepts/connectors.md#using-connectors) to learn more about using connectors. The values and specification sample below provide configuration details specific to the Intercom source connector.

### Properties

#### Endpoint

The properties in the table below reflect manual authentication using the CLI. In the Flow web app,
you'll sign in directly and won't need the access token.

| Property | Title | Description | Type | Required/Default |
|---|---|---|---|---|
| **`/credentials/access_token`** | Access Token | Intercom Access token. | string | Required |
| **`/credentials/credentials_title`** | Credentials | Name of the credentials set | string | Required, `"Private App Credentials"` |
| `/start_date` | Start date | UTC date and time in the format 2017-01-25T00:00:00Z. Any data before this date will not be replicated. | string | 30 days before the present date |
| `/advanced/window_size` | Window size | Window size in days for incrementals streams. Typically left as the default unless more frequent checkpoints are desired. | integer | 5 |

#### Bindings

| Property | Title | Description | Type | Required/Default |
|---|---|---|---|---|
| **`/name`** | Data resource | Name of the data resource. | string | Required |
| `/interval` | Interval | Interval between data syncs | string |          |


### Sample

```yaml
captures:
  ${PREFIX}/${CAPTURE_NAME}:
    endpoint:
      connector:
        image: ghcr.io/estuary/source-intercom-native:dev
        config:
            credentials:
                credentials_title: Private App Credentials
                access_token: <secret>
            start_date: "2024-12-13T12:00:00Z"
    bindings:
      - resource:
          name: admins
        target: ${PREFIX}/admins
      - resource:
          name: companies
        target: ${PREFIX}/companies
      - resource:
          name: company_segments
        target: ${PREFIX}/companysegments
      - resource:
          name: conversations
        target: ${PREFIX}/conversations
      - resource:
          name: conversation_parts
        target: ${PREFIX}/conversationparts
      - resource:
          name: contacts
        target: ${PREFIX}/contacts
      - resource:
          name: company_attributes
        target: ${PREFIX}/companyattributes
      - resource:
          name: contact_attributes
        target: ${PREFIX}/contactattributes
      - resource:
          name: segments
        target: ${PREFIX}/segments
      - resource:
          name: tags
        target: ${PREFIX}/tags
      - resource:
          name: teams
        target: ${PREFIX}/teams
```
