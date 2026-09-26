using UnityEngine;

public class Animal : MonoBehaviour
{
    [Header("Animal Info")]
    public string animalName;
    public string displayName; // German name shown to player
    public AnimalType type;
    public string[] acceptedFood;

    [Header("State")]
    public bool isInCorrectEnclosure;
    public bool isFed;

    [Header("Movement")]
    public float wanderRadius = 3f;
    public float moveSpeed = 1.5f;

    private Vector3 wanderTarget;
    private Vector3 startPosition;
    private float wanderTimer;

    public enum AnimalType
    {
        Hippo,
        Panda,
        Zebra,
        Koala,
        Elephant,
        Goldfish,
        Monkey
    }

    void Start()
    {
        startPosition = transform.position;
        PickNewWanderTarget();
    }

    void Update()
    {
        Wander();
    }

    void Wander()
    {
        wanderTimer -= Time.deltaTime;

        Vector3 direction = (wanderTarget - transform.position);
        direction.y = 0;

        if (direction.magnitude < 0.3f || wanderTimer <= 0)
        {
            PickNewWanderTarget();
            return;
        }

        transform.position += direction.normalized * moveSpeed * Time.deltaTime;
        transform.rotation = Quaternion.Slerp(
            transform.rotation,
            Quaternion.LookRotation(direction),
            Time.deltaTime * 3f
        );
    }

    void PickNewWanderTarget()
    {
        Vector2 randomCircle = Random.insideUnitCircle * wanderRadius;
        wanderTarget = startPosition + new Vector3(randomCircle.x, 0, randomCircle.y);
        wanderTimer = Random.Range(2f, 5f);
    }

    public bool TryFeed(string foodName)
    {
        foreach (string accepted in acceptedFood)
        {
            if (accepted.ToLower() == foodName.ToLower())
            {
                isFed = true;
                Debug.Log($"{displayName} wurde gefüttert mit {foodName}!");
                GameManager.Instance.AddScore(10);
                return true;
            }
        }
        Debug.Log($"{displayName} mag kein {foodName}.");
        return false;
    }

    public void PlaceInEnclosure(Enclosure enclosure)
    {
        if (enclosure.acceptedAnimalType == type)
        {
            isInCorrectEnclosure = true;
            startPosition = enclosure.animalSpawnPoint.position;
            transform.position = startPosition;
            GameManager.Instance.AddScore(20);
            Debug.Log($"{displayName} ist jetzt im richtigen Gehege!");
        }
    }
}
