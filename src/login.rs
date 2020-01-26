use spotify_api::authentication::{Scope, SpotifyOAuth};

pub async fn get_login_url() -> String {
    let scopes = vec![
        Scope::UserReadPrivate,
        Scope::UserReadBirthdate,
        Scope::UserReadEmail,
        Scope::Streaming,
        Scope::AppRemoteControl,
        Scope::UserTopRead,
        Scope::UserReadRecentlyPlayed,
        Scope::UserLibraryRead,
        Scope::UserLibraryModify,
        Scope::PlaylistReadCollaborative,
        Scope::PlaylistReadPrivate,
        Scope::PlaylistModifyPublic,
        Scope::PlaylistModifyPrivate,
        Scope::UserReadCurrentlyPlaying,
        Scope::UserReadPlaybackState,
        Scope::UserModifyPlaybackState,
        Scope::UserFollowRead,
        Scope::UserFollowModify,
    ];

    let mut oauth = SpotifyOAuth::new();
    oauth.set_scopes(&scopes);
    oauth.generate_auth_url().unwrap()
}
